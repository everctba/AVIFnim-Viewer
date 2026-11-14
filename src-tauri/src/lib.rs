mod assoc;
mod cache;
mod config;
mod pnm;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use base64::Engine as _;
use serde::Serialize;
use tauri::{Emitter, Manager};

use cache::Cache;
use config::Settings;

const SUPPORTED_EXTS: &[&str] = &[
    "avif", "webp", "gif", "ppm", "pgm", "pbm", "pnm", "png", "jpg", "jpeg",
];
const MAX_FILE_BYTES: u64 = 256 * 1024 * 1024;

// URLs ainda pendentes de definicao pelo owner (issues avifnim-xhk / avifnim-b7e).
const REPO_URL: &str = "https://github.com/everctba/AVIFnim-Viewer";
const SITE_URL: &str = "";
const AVIFNIM_URL: &str = "";

pub struct AppState {
    pub cache: Mutex<Cache>,
    pub settings: Mutex<Settings>,
    pub data_dir: PathBuf,
}

impl AppState {
    fn new(data_dir: PathBuf) -> Self {
        let settings = config::load(&data_dir);
        let cap = (settings.prefetch_mb as usize) * 1024 * 1024;
        Self {
            cache: Mutex::new(Cache::new(cap)),
            settings: Mutex::new(settings),
            data_dir,
        }
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub path: String,
    pub name: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageData {
    pub path: String,
    pub name: String,
    pub format: String,
    pub mime: String,
    pub size: u64,
    pub data_url: String,
    pub width: u32,
    pub height: u32,
    pub animated: bool,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub email: String,
    pub license: String,
    pub repo_url: String,
    pub site_url: String,
    pub avifnim_url: String,
    /// windows | macos | linux
    pub platform: String,
}

fn ext_lower(path: &Path) -> String {
    path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
}

fn is_supported(path: &Path) -> bool {
    let ext = ext_lower(path);
    SUPPORTED_EXTS.contains(&ext.as_str()) && path.is_file()
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string()
}

fn format_label(ext: &str) -> &'static str {
    match ext {
        "avif" => "AVIF",
        "webp" => "WebP",
        "gif" => "GIF",
        "ppm" | "pgm" | "pbm" | "pnm" => "PPM/PNM",
        "png" => "PNG",
        "jpg" | "jpeg" => "JPEG",
        _ => "Desconhecido",
    }
}

fn mime_for(ext: &str) -> &'static str {
    match ext {
        "avif" => "image/avif",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "ppm" | "pgm" | "pbm" | "pnm" => "image/x-portable-anymap",
        _ => "application/octet-stream",
    }
}

fn b64(bytes: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

fn read_file_cached(state: &AppState, path: &Path) -> Result<Arc<Vec<u8>>, String> {
    let key = path.to_string_lossy().into_owned();
    {
        let mut cache = state
            .cache
            .lock()
            .map_err(|_| "Cache indisponivel".to_string())?;
        if let Some(data) = cache.get(&key) {
            return Ok(data);
        }
    }
    let meta = std::fs::metadata(path)
        .map_err(|e| format!("Nao foi possivel acessar '{}': {e}", path.display()))?;
    if meta.len() > MAX_FILE_BYTES {
        return Err(format!(
            "Arquivo muito grande ({} MB; limite de 256 MB)",
            meta.len() / (1024 * 1024)
        ));
    }
    let bytes = std::fs::read(path)
        .map_err(|e| format!("Nao foi possivel ler '{}': {e}", path.display()))?;
    let arc = Arc::new(bytes);
    let mut cache = state
        .cache
        .lock()
        .map_err(|_| "Cache indisponivel".to_string())?;
    cache.put(key, arc.clone());
    Ok(arc)
}

fn find_supported_arg(argv: &[String]) -> Option<String> {
    argv.iter().skip(1).find_map(|arg| {
        let path = Path::new(arg);
        if is_supported(path) {
            Some(path.to_string_lossy().into_owned())
        } else {
            None
        }
    })
}

#[tauri::command]
fn initial_arg() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    find_supported_arg(&args)
}

#[tauri::command]
fn open_file_dialog() -> Option<String> {
    rfd::FileDialog::new()
        .set_title("Abrir imagem")
        .add_filter(
            "Imagens suportadas",
            &[
                "avif", "webp", "gif", "ppm", "pgm", "pbm", "pnm", "png", "jpg", "jpeg",
            ],
        )
        .add_filter("AVIF", &["avif"])
        .add_filter("WebP", &["webp"])
        .add_filter("GIF", &["gif"])
        .add_filter("PPM/PNM", &["ppm", "pgm", "pbm", "pnm"])
        .pick_file()
        .map(|p| p.to_string_lossy().into_owned())
}

#[tauri::command]
fn load_image(path: String, state: tauri::State<'_, AppState>) -> Result<ImageData, String> {
    let file = PathBuf::from(&path);
    if !file.is_file() {
        return Err(format!("Arquivo nao encontrado: {}", file.display()));
    }
    let ext = ext_lower(&file);
    if !SUPPORTED_EXTS.contains(&ext.as_str()) {
        return Err(format!("Formato nao suportado: .{ext}"));
    }
    let bytes = read_file_cached(&state, &file)?;
    let (payload, mime, width, height) = if pnm::is_pnm_ext(&ext) {
        let (png, w, h) = pnm::decode_pnm_to_png(&bytes)?;
        (png, "image/png".to_string(), w, h)
    } else {
        (bytes.as_ref().clone(), mime_for(&ext).to_string(), 0, 0)
    };
    let size = std::fs::metadata(&file)
        .map(|m| m.len())
        .unwrap_or(payload.len() as u64);
    let data_url = format!("data:{};base64,{}", mime, b64(&payload));
    Ok(ImageData {
        path: file.to_string_lossy().into_owned(),
        name: file_name(&file),
        format: format_label(&ext).to_string(),
        mime,
        size,
        data_url,
        width,
        height,
        animated: matches!(ext.as_str(), "avif" | "webp" | "gif"),
    })
}

#[tauri::command]
fn list_siblings(path: String) -> Result<Vec<FileEntry>, String> {
    let file = PathBuf::from(&path);
    let dir = file
        .parent()
        .ok_or_else(|| "Arquivo sem pasta pai".to_string())?;
    let mut entries = Vec::new();
    let read = std::fs::read_dir(dir)
        .map_err(|e| format!("Nao foi possivel listar '{}': {e}", dir.display()))?;
    for entry in read.flatten() {
        let p = entry.path();
        if is_supported(&p) {
            entries.push(FileEntry {
                path: p.to_string_lossy().into_owned(),
                name: file_name(&p),
            });
        }
    }
    entries.sort_by(|a, b| natural_cmp(&a.name, &b.name));
    Ok(entries)
}

#[tauri::command]
fn prefetch(paths: Vec<String>, state: tauri::State<'_, AppState>) {
    for path in paths {
        let p = PathBuf::from(&path);
        if is_supported(&p) {
            let _ = read_file_cached(&state, &p);
        }
    }
}

#[tauri::command]
fn clear_cache(state: tauri::State<'_, AppState>) {
    if let Ok(mut cache) = state.cache.lock() {
        cache.clear();
    }
}

#[tauri::command]
fn get_settings(state: tauri::State<'_, AppState>) -> Settings {
    state.settings.lock().map(|s| s.clone()).unwrap_or_default()
}

#[tauri::command]
fn save_settings(
    settings: Settings,
    state: tauri::State<'_, AppState>,
) -> Result<Settings, String> {
    let normalized = settings.normalized();
    config::save(&state.data_dir, &normalized)?;
    let cap = (normalized.prefetch_mb as usize) * 1024 * 1024;
    if let Ok(mut cache) = state.cache.lock() {
        cache.set_cap(cap);
    }
    if let Ok(mut current) = state.settings.lock() {
        *current = normalized.clone();
    }
    Ok(normalized)
}

#[tauri::command]
fn record_view(state: tauri::State<'_, AppState>) -> Result<u32, String> {
    let mut settings = state
        .settings
        .lock()
        .map_err(|_| "Configuracao indisponivel".to_string())?;
    settings.view_count = settings.view_count.saturating_add(1);
    let count = settings.view_count;
    let snapshot = settings.clone();
    drop(settings);
    config::save(&state.data_dir, &snapshot)?;
    Ok(count)
}

#[tauri::command]
fn app_info() -> AppInfo {
    AppInfo {
        name: "AVIFnim Viewer".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        author: "Everson Mayer".into(),
        email: "mayerdesigner@gmail.com".into(),
        license: "MIT".into(),
        repo_url: REPO_URL.into(),
        site_url: SITE_URL.into(),
        avifnim_url: AVIFNIM_URL.into(),
        platform: std::env::consts::OS.into(),
    }
}

/// Abre a pasta do arquivo no gerenciador de arquivos do sistema (cross-platform).
#[tauri::command]
fn reveal_in_folder(path: String) -> Result<(), String> {
    let file = PathBuf::from(&path);
    if !file.exists() {
        return Err(format!("Arquivo nao encontrado: {}", file.display()));
    }
    let folder = file
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| file.clone());

    #[cfg(target_os = "windows")]
    let result = std::process::Command::new("explorer.exe")
        .arg(&folder)
        .spawn();

    #[cfg(target_os = "macos")]
    let result = std::process::Command::new("open").arg(&folder).spawn();

    #[cfg(all(unix, not(target_os = "macos")))]
    let result = std::process::Command::new("xdg-open").arg(&folder).spawn();

    result
        .map(|_| ())
        .map_err(|e| format!("Falha ao abrir a pasta: {e}"))
}

/// Abre o dialogo "Abrir com..." do Windows. Em outras plataformas retorna erro.
#[tauri::command]
fn open_with_dialog(path: String) -> Result<(), String> {
    let file = PathBuf::from(&path);
    if !file.exists() {
        return Err(format!("Arquivo nao encontrado: {}", file.display()));
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("rundll32.exe")
            .arg("shell32.dll,OpenAs_RunDLL")
            .arg(&file)
            .spawn()
            .map(|_| ())
            .map_err(|e| format!("Falha ao abrir 'Abrir com...': {e}"))
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("'Abrir com...' esta disponivel apenas no Windows".into())
    }
}

#[tauri::command]
fn open_url(url: String) -> Result<(), String> {
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return Err("URL invalida".into());
    }
    open::that(url).map_err(|e| format!("Falha ao abrir o navegador: {e}"))
}

#[tauri::command]
fn get_association_states() -> HashMap<String, bool> {
    assoc::get_states()
}

#[tauri::command]
fn set_association(group: String, enabled: bool) -> Result<bool, String> {
    assoc::set_association(&group, enabled)
}

#[tauri::command]
fn avifnim_installed() -> bool {
    assoc::avifnim_installed()
}

#[tauri::command]
fn open_default_apps_settings() -> Result<(), String> {
    #[cfg(windows)]
    {
        open::that("ms-settings:defaultapps")
            .map_err(|e| format!("Falha ao abrir as configuracoes do Windows: {e}"))
    }
    #[cfg(not(windows))]
    {
        Err("Recurso disponivel apenas no Windows".into())
    }
}

#[tauri::command]
fn set_window_theme(window: tauri::WebviewWindow, dark: bool) {
    let theme = if dark {
        tauri::Theme::Dark
    } else {
        tauri::Theme::Light
    };
    let _ = window.set_theme(Some(theme));
}

#[tauri::command]
fn set_window_title(window: tauri::WebviewWindow, title: String) {
    let _ = window.set_title(&title);
}

/// Modo de benchmark (ativado por AVIFNIM_BENCH=1): usado por scripts de medicao.
#[tauri::command]
fn bench_mode() -> bool {
    std::env::var("AVIFNIM_BENCH")
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Anexa uma linha JSON ao bench.log do app data (usado pelo modo de benchmark).
#[tauri::command]
fn bench_log(line: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    use std::io::Write;
    let path = state.data_dir.join("bench.log");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| format!("Falha ao abrir bench.log: {e}"))?;
    writeln!(file, "{line}").map_err(|e| format!("Falha ao escrever bench.log: {e}"))?;
    Ok(())
}

/// Ordenacao natural: frame_2 antes de frame_10.
fn natural_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let av: Vec<char> = a.chars().collect();
    let bv: Vec<char> = b.chars().collect();
    let (mut i, mut j) = (0usize, 0usize);
    while i < av.len() && j < bv.len() {
        if av[i].is_ascii_digit() && bv[j].is_ascii_digit() {
            let (na, ni) = parse_chars_num(&av, i);
            let (nb, nj) = parse_chars_num(&bv, j);
            if na != nb {
                return na.cmp(&nb);
            }
            i = ni;
            j = nj;
        } else {
            let ca = av[i].to_ascii_lowercase();
            let cb = bv[j].to_ascii_lowercase();
            if ca != cb {
                return ca.cmp(&cb);
            }
            i += 1;
            j += 1;
        }
    }
    av.len().cmp(&bv.len())
}

fn parse_chars_num(chars: &[char], mut i: usize) -> (u128, usize) {
    let mut value: u128 = 0;
    while i < chars.len() && chars[i].is_ascii_digit() {
        value = value
            .saturating_mul(10)
            .saturating_add((chars[i] as u8 - b'0') as u128);
        i += 1;
    }
    (value, i)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            if let Some(path) = find_supported_arg(&argv) {
                let _ = app.emit("open-path", path);
            }
        }))
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            let _ = std::fs::create_dir_all(&data_dir);
            app.manage(AppState::new(data_dir));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            initial_arg,
            open_file_dialog,
            load_image,
            list_siblings,
            prefetch,
            clear_cache,
            get_settings,
            save_settings,
            record_view,
            app_info,
            open_url,
            get_association_states,
            set_association,
            avifnim_installed,
            open_default_apps_settings,
            set_window_theme,
            set_window_title,
            bench_mode,
            bench_log,
            reveal_in_folder,
            open_with_dialog
        ])
        .build(tauri::generate_context!())
        .expect("erro ao inicializar o AVIFnim Viewer")
        .run(|app, event| handle_run_event(app, &event));
}

#[cfg(not(target_os = "macos"))]
fn handle_run_event(_app: &tauri::AppHandle, _event: &tauri::RunEvent) {}

#[cfg(target_os = "macos")]
fn handle_run_event(app: &tauri::AppHandle, event: &tauri::RunEvent) {
    if let tauri::RunEvent::Opened { urls } = event {
        for url in urls {
            if let Ok(path) = url.to_file_path() {
                let _ = app.emit("open-path", path.to_string_lossy().into_owned());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn natural_sort_orders_frames() {
        let mut names = vec!["frame_10.png", "frame_2.png", "frame_1.png", "frame_02.png"];
        names.sort_by(|a, b| natural_cmp(a, b));
        assert_eq!(
            names,
            vec!["frame_1.png", "frame_2.png", "frame_02.png", "frame_10.png"]
        );
    }

    #[test]
    fn format_and_mime_helpers() {
        assert_eq!(format_label("avif"), "AVIF");
        assert_eq!(format_label("pnm"), "PPM/PNM");
        assert_eq!(mime_for("webp"), "image/webp");
        assert_eq!(mime_for("jpg"), "image/jpeg");
        assert_eq!(ext_lower(Path::new("C:\\tmp\\FOTO.AVIF")), "avif");
    }

    #[test]
    fn finds_supported_file_argument() {
        let dir = std::env::temp_dir().join("avifnim-arg-test");
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("foto.avif");
        std::fs::write(&file, b"x").unwrap();
        let expected = file.to_string_lossy().into_owned();
        let args = vec![
            "avifnim-viewer.exe".to_string(),
            "--flag".to_string(),
            expected.clone(),
        ];
        assert_eq!(find_supported_arg(&args), Some(expected));
        let _ = std::fs::remove_file(&file);
    }

    #[test]
    fn ignores_unsupported_argument() {
        let args = vec!["app.exe".to_string(), "arquivo.txt".to_string()];
        assert_eq!(find_supported_arg(&args), None);
    }
}
