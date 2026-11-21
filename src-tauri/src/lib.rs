mod assoc;
#[cfg(not(target_os = "linux"))]
mod avif_fallback;
#[cfg(target_os = "linux")]
mod avif_native;
mod cache;
mod config;
#[cfg(target_os = "macos")]
mod macos_ls;
pub mod pnm;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
#[cfg(target_os = "linux")]
use std::sync::atomic::Ordering;
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

// URLs oficiais do projeto.
const REPO_URL: &str = "https://github.com/everctba/AVIFnim-Viewer";
const SITE_URL: &str = "https://everctba.github.io/avifnim-site";
const AVIFNIM_URL: &str = "https://everctba.github.io/avifnim-site";
// CTA "Conheça/Converter com AVIFnim": abre direto a secao de compra.
const AVIFNIM_SALES_URL: &str = "https://everctba.github.io/avifnim-site/#comprar";
// Pagina de doacao (PayPal). O sufixo "/" permite query string (business, amount, currency).
const PAYPAL_URL: &str = "https://www.paypal.com/donate/";

/// Allowlist de URLs que o app pode abrir no navegador.
/// Se um renderer for comprometido, `open_url` nao pode ser usado para exfiltrar dados
/// abrindo URLs arbitrarias.
const ALLOWED_URLS: &[&str] = &[
    REPO_URL,
    SITE_URL,
    AVIFNIM_URL,
    AVIFNIM_SALES_URL,
    PAYPAL_URL,
    "mailto:mayerdesigner@gmail.com",
];

fn is_allowed_url(url: &str) -> bool {
    if url.is_empty() || url.len() > 2048 {
        return false;
    }
    ALLOWED_URLS.iter().any(|allowed| {
        if allowed.starts_with("mailto:") {
            url == *allowed
        } else {
            url == *allowed
                || url.starts_with(&format!("{allowed}/"))
                || url.starts_with(&format!("{allowed}?"))
                || url.starts_with(&format!("{allowed}#"))
        }
    })
}

pub struct AppState {
    pub cache: Mutex<Cache>,
    pub settings: Mutex<Settings>,
    pub data_dir: PathBuf,
    /// Arquivo recebido pelo macOS via Apple Event antes do frontend carregar.
    pub pending_open: Mutex<Option<String>>,
    /// Cancela a stream AVIF nativa ao navegar para outro arquivo.
    pub avif_cancel: Mutex<Option<Arc<AtomicBool>>>,
}

impl AppState {
    fn new(data_dir: PathBuf) -> Self {
        let settings = config::load(&data_dir);
        let cap = (settings.prefetch_mb as usize) * 1024 * 1024;
        Self {
            cache: Mutex::new(Cache::new(cap)),
            settings: Mutex::new(settings),
            data_dir,
            pending_open: Mutex::new(None),
            avif_cancel: Mutex::new(None),
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
    /// true quando o app esta rodando como pacote MSIX (Microsoft Store).
    pub msix: bool,
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

fn app_lang(state: &AppState) -> String {
    state
        .settings
        .lock()
        .map(|s| s.lang.clone())
        .unwrap_or_else(|_| "pt".to_string())
}

fn localized(lang: &str, pt: String, en: String) -> String {
    if lang == "en" {
        en
    } else {
        pt
    }
}

/// Traduz mensagens fixas do backend (associacoes/recursos de plataforma) para EN.
fn localize_known_error(err: String, lang: &str) -> String {
    if lang != "en" {
        return err;
    }
    const EXACT: &[(&str, &str)] = &[
        (
            "Associações de arquivo são gerenciadas pelo sistema operacional nesta plataforma",
            "File associations are managed by the operating system on this platform",
        ),
        (
            "'Abrir com…' está disponível apenas no Windows",
            "'Open with…' is only available on Windows",
        ),
        (
            "Recurso disponível apenas no Windows",
            "Feature only available on Windows",
        ),
        (
            "URL não permitida pelo AVIFnim Viewer",
            "URL not allowed by AVIFnim Viewer",
        ),
        (
            "Não foi possível descobrir o caminho do executável",
            "Could not determine the executable path",
        ),
    ];
    for (pt, en) in EXACT {
        if err == *pt {
            return (*en).to_string();
        }
    }
    if let Some(rest) = err.strip_prefix("Grupo de associação desconhecido: ") {
        return format!("Unknown association group: {rest}");
    }
    if let Some(rest) = err.strip_prefix("Falha ao acessar HKCU\\Software\\Classes: ") {
        return format!("Failed to access HKCU\\Software\\Classes: {rest}");
    }
    if let Some(rest) = err.strip_prefix("Falha ao registrar ") {
        return format!("Failed to register {rest}");
    }
    if let Some(rest) = err.strip_prefix("Falha ao gravar ProgID: ") {
        return format!("Failed to write ProgID: {rest}");
    }
    if let Some(rest) = err.strip_prefix("Falha ao gravar ícone: ") {
        return format!("Failed to write icon: {rest}");
    }
    if let Some(rest) = err.strip_prefix("Falha ao gravar comando: ") {
        return format!("Failed to write command: {rest}");
    }
    if let Some(rest) = err.strip_prefix("Falha ao adicionar abre-com para ") {
        return format!("Failed to add open-with for {rest}");
    }
    err
}

fn read_file_cached(state: &AppState, path: &Path, lang: &str) -> Result<Arc<Vec<u8>>, String> {
    let key = path.to_string_lossy().into_owned();
    {
        let mut cache = state.cache.lock().map_err(|_| {
            localized(
                lang,
                "Cache indisponível".to_string(),
                "Cache unavailable".to_string(),
            )
        })?;
        if let Some(data) = cache.get(&key) {
            return Ok(data);
        }
    }
    let meta = std::fs::metadata(path).map_err(|e| {
        localized(
            lang,
            format!("Não foi possível acessar '{}': {e}", path.display()),
            format!("Could not access '{}': {e}", path.display()),
        )
    })?;
    if meta.len() > MAX_FILE_BYTES {
        return Err(localized(
            lang,
            format!(
                "Arquivo muito grande ({} MB; limite de 256 MB)",
                meta.len() / (1024 * 1024)
            ),
            format!(
                "File too large ({} MB; 256 MB limit)",
                meta.len() / (1024 * 1024)
            ),
        ));
    }
    let bytes = std::fs::read(path).map_err(|e| {
        localized(
            lang,
            format!("Não foi possível ler '{}': {e}", path.display()),
            format!("Could not read '{}': {e}", path.display()),
        )
    })?;
    let arc = Arc::new(bytes);
    let mut cache = state.cache.lock().map_err(|_| {
        localized(
            lang,
            "Cache indisponível".to_string(),
            "Cache unavailable".to_string(),
        )
    })?;
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
fn initial_arg(state: tauri::State<'_, AppState>) -> Option<String> {
    // No macOS o duplo clique no Finder chega por Apple Event (RunEvent::Opened),
    // que pode disparar antes do frontend registrar o listener.
    if let Ok(mut pending) = state.pending_open.lock() {
        if let Some(path) = pending.take() {
            return Some(path);
        }
    }
    let args: Vec<String> = std::env::args().collect();
    find_supported_arg(&args)
}

#[tauri::command]
fn open_file_dialog(state: tauri::State<'_, AppState>) -> Option<String> {
    let lang = app_lang(&state);
    let title = if lang == "en" {
        "Open image"
    } else {
        "Abrir imagem"
    };
    let all_filter = if lang == "en" {
        "Supported images"
    } else {
        "Imagens suportadas"
    };
    rfd::FileDialog::new()
        .set_title(title)
        .add_filter(
            all_filter,
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
    let lang = app_lang(&state);
    if !file.is_file() {
        return Err(localized(
            &lang,
            format!("Arquivo não encontrado: {}", file.display()),
            format!("File not found: {}", file.display()),
        ));
    }
    let ext = ext_lower(&file);
    if !SUPPORTED_EXTS.contains(&ext.as_str()) {
        return Err(localized(
            &lang,
            format!("Formato não suportado: .{ext}"),
            format!("Unsupported format: .{ext}"),
        ));
    }
    let bytes = read_file_cached(&state, &file, &lang)?;
    let (payload, mime, width, height) = if pnm::is_pnm_ext(&ext) {
        let (png, w, h) = pnm::decode_pnm_to_png_lang(&bytes, &lang)?;
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

fn read_avif_fallback_bytes(
    path: &str,
    state: &AppState,
) -> Result<(Arc<Vec<u8>>, String), String> {
    let file = PathBuf::from(path);
    let lang = app_lang(state);
    if !file.is_file() {
        return Err(localized(
            &lang,
            format!("Arquivo não encontrado: {}", file.display()),
            format!("File not found: {}", file.display()),
        ));
    }
    let ext = ext_lower(&file);
    if ext != "avif" {
        return Err(localized(
            &lang,
            format!("Formato não suportado: .{ext}"),
            format!("Unsupported format: .{ext}"),
        ));
    }
    let bytes = read_file_cached(state, &file, &lang)?;
    Ok((bytes, lang))
}

#[cfg(target_os = "linux")]
#[tauri::command]
fn decode_avif_fallback(
    path: String,
    state: tauri::State<'_, AppState>,
) -> Result<avif_native::AvifPayload, String> {
    let (bytes, lang) = read_avif_fallback_bytes(&path, &state)?;
    avif_native::decode(&bytes, &lang)
}

#[cfg(not(target_os = "linux"))]
#[tauri::command]
fn decode_avif_fallback(
    path: String,
    state: tauri::State<'_, AppState>,
) -> Result<avif_fallback::AvifPayload, String> {
    let (bytes, lang) = read_avif_fallback_bytes(&path, &state)?;
    avif_fallback::decode(&bytes, &lang)
}

#[cfg(target_os = "linux")]
#[tauri::command]
fn start_avif_stream(
    path: String,
    channel: tauri::ipc::Channel<tauri::ipc::InvokeResponseBody>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let (bytes, lang) = read_avif_fallback_bytes(&path, &state)?;
    let cancel = Arc::new(AtomicBool::new(false));
    if let Ok(mut current) = state.avif_cancel.lock() {
        if let Some(previous) = current.take() {
            previous.store(true, Ordering::Relaxed);
        }
        *current = Some(cancel.clone());
    }
    avif_native::spawn_stream(bytes, lang, cancel, channel);
    Ok(())
}

#[cfg(target_os = "linux")]
#[tauri::command]
fn stop_avif_stream(state: tauri::State<'_, AppState>) {
    if let Ok(mut current) = state.avif_cancel.lock() {
        if let Some(cancel) = current.take() {
            cancel.store(true, Ordering::Relaxed);
        }
    }
}

#[cfg(not(target_os = "linux"))]
#[tauri::command]
fn start_avif_stream(
    _path: String,
    _channel: tauri::ipc::Channel<tauri::ipc::InvokeResponseBody>,
) -> Result<(), String> {
    Err("Native AVIF streaming is Linux-only".to_string())
}

#[cfg(not(target_os = "linux"))]
#[tauri::command]
fn stop_avif_stream() {}

#[tauri::command]
fn list_siblings(
    path: String,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<FileEntry>, String> {
    let lang = app_lang(&state);
    let file = PathBuf::from(&path);
    let dir = file.parent().ok_or_else(|| {
        localized(
            &lang,
            "Arquivo sem pasta pai".to_string(),
            "File has no parent folder".to_string(),
        )
    })?;
    let mut entries = Vec::new();
    let read = std::fs::read_dir(dir).map_err(|e| {
        localized(
            &lang,
            format!("Não foi possível listar '{}': {e}", dir.display()),
            format!("Could not list '{}': {e}", dir.display()),
        )
    })?;
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
    let lang = app_lang(&state);
    for path in paths {
        let p = PathBuf::from(&path);
        if is_supported(&p) {
            let _ = read_file_cached(&state, &p, &lang);
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
        .map_err(|_| "Configuração indisponível".to_string())?;
    settings.view_count = settings.view_count.saturating_add(1);
    let count = settings.view_count;
    let snapshot = settings.clone();
    drop(settings);
    config::save(&state.data_dir, &snapshot)?;
    Ok(count)
}

#[cfg(windows)]
fn is_msix_packaged() -> bool {
    #[link(name = "kernel32")]
    extern "system" {
        fn GetCurrentPackageFullName(length: *mut u32, package_full_name: *mut u16) -> i32;
    }
    // ERROR_INSUFFICIENT_BUFFER (122): empacotado, mas o buffer de nome e pequeno.
    // APPMODEL_ERROR_NO_PACKAGE (15700): rodando fora de um pacote MSIX.
    unsafe {
        let mut length = 0u32;
        GetCurrentPackageFullName(&mut length, std::ptr::null_mut()) == 122
    }
}

#[cfg(not(windows))]
fn is_msix_packaged() -> bool {
    false
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
        avifnim_url: AVIFNIM_SALES_URL.into(),
        platform: std::env::consts::OS.into(),
        msix: is_msix_packaged(),
    }
}

/// Abre a pasta do arquivo no gerenciador de arquivos do sistema (cross-platform).
#[tauri::command]
fn reveal_in_folder(path: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let lang = app_lang(&state);
    let file = PathBuf::from(&path);
    if !is_supported(&file) {
        return Err(localized(
            &lang,
            format!("Arquivo não suportado: {}", file.display()),
            format!("Unsupported file: {}", file.display()),
        ));
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

    result.map(|_| ()).map_err(|e| {
        localized(
            &lang,
            format!("Falha ao abrir a pasta: {e}"),
            format!("Failed to open folder: {e}"),
        )
    })
}

/// Abre o dialogo "Abrir com..." do Windows. Em outras plataformas retorna erro.
#[tauri::command]
fn open_with_dialog(path: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let lang = app_lang(&state);
    let file = PathBuf::from(&path);
    if !is_supported(&file) {
        return Err(localized(
            &lang,
            format!("Arquivo não suportado: {}", file.display()),
            format!("Unsupported file: {}", file.display()),
        ));
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("rundll32.exe")
            .arg("shell32.dll,OpenAs_RunDLL")
            .arg(&file)
            .spawn()
            .map(|_| ())
            .map_err(|e| {
                localized(
                    &lang,
                    format!("Falha ao abrir 'Abrir com…': {e}"),
                    format!("Failed to open 'Open with…': {e}"),
                )
            })
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err(localized(
            &lang,
            "'Abrir com…' está disponível apenas no Windows".to_string(),
            "'Open with…' is only available on Windows".to_string(),
        ))
    }
}

#[tauri::command]
fn open_url(url: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let lang = app_lang(&state);
    if !is_allowed_url(&url) {
        return Err(localized(
            &lang,
            "URL não permitida pelo AVIFnim Viewer".to_string(),
            "URL not allowed by AVIFnim Viewer".to_string(),
        ));
    }
    open::that_detached(url).map_err(|e| {
        localized(
            &lang,
            format!("Falha ao abrir o navegador: {e}"),
            format!("Failed to open browser: {e}"),
        )
    })
}

#[tauri::command]
fn get_association_states() -> HashMap<String, bool> {
    assoc::get_states()
}

#[tauri::command]
fn set_association(
    group: String,
    enabled: bool,
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    let lang = app_lang(&state);
    assoc::set_association(&group, enabled).map_err(|err| localize_known_error(err, &lang))
}

#[tauri::command]
fn avifnim_installed() -> bool {
    assoc::avifnim_installed()
}

#[tauri::command]
fn open_default_apps_settings(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let lang = app_lang(&state);
    #[cfg(windows)]
    {
        open::that("ms-settings:defaultapps").map_err(|e| {
            localized(
                &lang,
                format!("Falha ao abrir as configurações do Windows: {e}"),
                format!("Failed to open Windows settings: {e}"),
            )
        })
    }
    #[cfg(not(windows))]
    {
        Err(localized(
            &lang,
            "Recurso disponível apenas no Windows".to_string(),
            "Feature only available on Windows".to_string(),
        ))
    }
}

/// macOS: registra o app como handler padrao para os formatos suportados.
#[tauri::command]
fn set_default_file_associations(app: tauri::AppHandle) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let bundle_id = app.config().identifier.clone();
        macos_ls::set_default_handlers(&bundle_id)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = app;
        Err("Feature only available on macOS".to_string())
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
            decode_avif_fallback,
            start_avif_stream,
            stop_avif_stream,
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
            set_default_file_associations,
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
                if !is_supported(&path) {
                    continue;
                }
                let path_string = path.to_string_lossy().into_owned();
                if let Some(state) = app.try_state::<AppState>() {
                    if let Ok(mut pending) = state.pending_open.lock() {
                        if pending.is_none() {
                            *pending = Some(path_string.clone());
                        }
                    }
                }
                let _ = app.emit("open-path", path_string);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn localizes_platform_errors_to_english() {
        let cases = [
            (
                "Associações de arquivo são gerenciadas pelo sistema operacional nesta plataforma",
                "File associations are managed by the operating system",
            ),
            (
                "Recurso disponível apenas no Windows",
                "Feature only available on Windows",
            ),
            (
                "'Abrir com…' está disponível apenas no Windows",
                "is only available on Windows",
            ),
            (
                "URL não permitida pelo AVIFnim Viewer",
                "URL not allowed by AVIFnim Viewer",
            ),
        ];
        for (pt, expected) in cases {
            let out = localize_known_error(pt.to_string(), "en");
            assert!(out.contains(expected), "{pt} -> {out}");
        }
    }

    #[test]
    fn keeps_portuguese_platform_errors() {
        let pt = "Recurso disponível apenas no Windows";
        assert_eq!(localize_known_error(pt.to_string(), "pt"), pt);
    }

    #[test]
    fn localizes_assoc_group_error() {
        let out = localize_known_error("Grupo de associação desconhecido: avif".to_string(), "en");
        assert_eq!(out, "Unknown association group: avif");
    }

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

    #[test]
    fn url_allowlist_blocks_external_urls() {
        assert!(is_allowed_url(REPO_URL));
        assert!(is_allowed_url(
            "https://github.com/everctba/AVIFnim-Viewer/releases/tag/v0.1.1"
        ));
        assert!(is_allowed_url("mailto:mayerdesigner@gmail.com"));
        assert!(is_allowed_url(SITE_URL));
        assert!(is_allowed_url(AVIFNIM_URL));
        assert!(is_allowed_url(AVIFNIM_SALES_URL));
        assert!(is_allowed_url("https://everctba.github.io/avifnim-site/"));
        assert!(is_allowed_url(PAYPAL_URL));
        assert!(is_allowed_url(
            "https://www.paypal.com/donate/?business=mayerdesigner%40gmail.com&currency_code=USD&amount=5.00"
        ));

        assert!(!is_allowed_url("https://example.com/exfil?dados=secreto"));
        assert!(!is_allowed_url(
            "https://everctba.github.io/avifnim-site-other"
        ));
        assert!(!is_allowed_url("https://www.paypal.com/donate.evil/?x=1"));
        assert!(!is_allowed_url("https://www.paypal.com/signin"));
        assert!(!is_allowed_url("https://github.com/everctba/outro-repo"));
        assert!(!is_allowed_url("http://github.com/everctba/AVIFnim-Viewer"));
        assert!(!is_allowed_url("mailto:outro@example.com"));
        assert!(!is_allowed_url("javascript:alert(1)"));
        assert!(!is_allowed_url("file:///C:/Windows/System32/calc.exe"));
        assert!(!is_allowed_url(""));
    }

    #[test]
    fn supported_path_validation_rejects_non_images() {
        let dir = std::env::temp_dir().join("avifnim-supported-test");
        std::fs::create_dir_all(&dir).unwrap();
        let txt = dir.join("nota.txt");
        std::fs::write(&txt, b"x").unwrap();
        let img = dir.join("foto.avif");
        std::fs::write(&img, b"x").unwrap();
        assert!(!is_supported(&txt));
        assert!(is_supported(&img));
        let _ = std::fs::remove_file(&txt);
        let _ = std::fs::remove_file(&img);
    }
}
