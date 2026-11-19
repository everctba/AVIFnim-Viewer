use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Configuracoes persistentes em JSON no app data.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct Settings {
    /// auto | light | dark
    pub theme: String,
    /// pt | en
    pub lang: String,
    /// numero de vizinhos prefetch para cada lado
    pub prefetch_n: u32,
    /// teto de memoria do cache de prefetch em MB
    pub prefetch_mb: u32,
    /// dark | light | gray
    pub background: String,
    pub checkerboard: bool,
    /// 30 | 60 (FPS de sequencias)
    pub seq_fps: u32,
    pub show_tips: bool,
    pub promo_seen: bool,
    pub banner_dismissed: bool,
    /// Reseta zoom/pan/fit ao trocar de arquivo (FR-05: configutavel).
    pub reset_view_on_navigate: bool,
    pub view_count: u32,
    /// intencao do usuario por grupo: avif | webp | gif | pnm
    pub associations: HashMap<String, bool>,
}

impl Default for Settings {
    fn default() -> Self {
        let mut associations = HashMap::new();
        for key in ["avif", "webp", "gif", "pnm"] {
            associations.insert(key.to_string(), false);
        }
        Self {
            theme: "dark".into(),
            lang: "pt".into(),
            prefetch_n: 3,
            prefetch_mb: 128,
            background: "dark".into(),
            checkerboard: false,
            seq_fps: 60,
            show_tips: true,
            promo_seen: false,
            banner_dismissed: false,
            reset_view_on_navigate: true,
            view_count: 0,
            associations,
        }
    }
}

impl Settings {
    pub fn normalized(mut self) -> Self {
        // Somente dois estados de tema: light | dark (auto/valores antigos viram dark).
        if self.theme != "light" && self.theme != "dark" {
            self.theme = "dark".into();
        }
        if !matches!(self.lang.as_str(), "pt" | "en") {
            self.lang = "pt".into();
        }
        if self.seq_fps != 30 && self.seq_fps != 60 {
            self.seq_fps = 60;
        }
        if !matches!(self.background.as_str(), "dark" | "light" | "gray") {
            self.background = "dark".into();
        }
        self.prefetch_n = self.prefetch_n.min(10);
        self.prefetch_mb = self.prefetch_mb.clamp(8, 1024);
        for key in ["avif", "webp", "gif", "pnm"] {
            self.associations.entry(key.to_string()).or_insert(false);
        }
        self
    }
}

fn settings_path(dir: &Path) -> std::path::PathBuf {
    dir.join("settings.json")
}

/// Converte o valor gravado pelo instalador ("en"/"pt") num idioma valido.
fn installer_lang_value(value: &str) -> Option<&'static str> {
    match value.trim().to_lowercase().as_str() {
        "en" => Some("en"),
        "pt" => Some("pt"),
        _ => None,
    }
}

/// Idioma escolhido no instalador NSIS (gravado em HKCU\Software\AVIFnim\AppLanguage).
#[cfg(target_os = "windows")]
fn installer_language() -> Option<&'static str> {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_READ};
    let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu
        .open_subkey_with_flags("Software\\AVIFnim", KEY_READ)
        .ok()?;
    let value: String = key.get_value("AppLanguage").ok()?;
    installer_lang_value(&value)
}

pub fn load(dir: &Path) -> Settings {
    let path = settings_path(dir);
    match std::fs::read(&path) {
        Ok(mut raw) => {
            // Tolera BOM UTF-8 (arquivos editados por algumas ferramentas no Windows).
            if raw.starts_with(&[0xEF, 0xBB, 0xBF]) {
                raw.drain(0..3);
            }
            serde_json::from_slice::<Settings>(&raw)
                .map(|s| s.normalized())
                .unwrap_or_default()
        }
        Err(_) => {
            // Primeira execucao: respeita o idioma escolhido no instalador (Windows).
            let mut settings = Settings::default();
            #[cfg(target_os = "windows")]
            if let Some(lang) = installer_language() {
                settings.lang = lang.to_string();
            }
            settings.normalized()
        }
    }
}

pub fn save(dir: &Path, settings: &Settings) -> Result<(), String> {
    std::fs::create_dir_all(dir)
        .map_err(|e| format!("Falha ao criar pasta de configuracao: {e}"))?;
    let path = settings_path(dir);
    let tmp = path.with_extension("json.tmp");
    let raw = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Falha ao serializar configuracao: {e}"))?;
    std::fs::write(&tmp, raw).map_err(|e| format!("Falha ao gravar configuracao: {e}"))?;
    std::fs::rename(&tmp, &path).map_err(|e| format!("Falha ao salvar configuracao: {e}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(name);
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn loads_settings_with_utf8_bom() {
        let dir = temp_dir("avifnim-config-bom");
        let mut bytes = vec![0xEF, 0xBB, 0xBF];
        bytes.extend_from_slice(br#"{"lang":"en","view_count":7}"#);
        std::fs::write(dir.join("settings.json"), bytes).unwrap();
        let settings = load(&dir);
        assert_eq!(settings.lang, "en");
        assert_eq!(settings.view_count, 7);
        assert_eq!(settings.prefetch_n, 3, "defaults devem ser preservados");
        assert!(!settings.checkerboard, "checkerboard desligado por padrao");
    }

    #[test]
    fn defaults_when_file_missing() {
        let dir = temp_dir("avifnim-config-missing");
        let settings = load(&dir);
        assert_eq!(settings.theme, "dark");
        assert_eq!(settings.seq_fps, 60);
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = temp_dir("avifnim-config-roundtrip");
        let settings = Settings {
            lang: "en".into(),
            prefetch_mb: 256,
            promo_seen: true,
            view_count: 9,
            ..Settings::default()
        };
        save(&dir, &settings).unwrap();
        let loaded = load(&dir);
        assert_eq!(loaded.lang, "en");
        assert_eq!(loaded.prefetch_mb, 256);
        assert!(loaded.promo_seen);
        assert_eq!(loaded.view_count, 9);
    }

    #[test]
    fn normalizes_out_of_range_values() {
        let settings = Settings {
            theme: "neon".into(),
            lang: "xx".into(),
            prefetch_mb: 99_999,
            seq_fps: 144,
            ..Settings::default()
        };
        let normalized = settings.normalized();
        assert_eq!(normalized.theme, "dark");
        assert_eq!(normalized.lang, "pt");
        assert_eq!(normalized.prefetch_mb, 1024);
        assert_eq!(normalized.seq_fps, 60);
    }

    #[test]
    fn parses_installer_language_value() {
        assert_eq!(installer_lang_value("en"), Some("en"));
        assert_eq!(installer_lang_value(" EN "), Some("en"));
        assert_eq!(installer_lang_value("pt"), Some("pt"));
        assert_eq!(installer_lang_value("PT-BR"), None);
        assert_eq!(installer_lang_value("es"), None);
        assert_eq!(installer_lang_value(""), None);
    }
}
