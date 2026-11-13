use std::collections::HashMap;

pub const GROUPS: &[&str] = &["avif", "webp", "gif", "pnm"];

#[cfg(windows)]
mod win {
    use super::*;
    use winreg::enums::*;
    use winreg::{RegKey, RegValue};

    fn group_exts(group: &str) -> &'static [&'static str] {
        match group {
            "avif" => &["avif"],
            "webp" => &["webp"],
            "gif" => &["gif"],
            "pnm" => &["ppm", "pgm", "pbm", "pnm"],
            _ => &[],
        }
    }

    fn progid(group: &str) -> String {
        format!("AVIFnimViewer.{group}")
    }

    fn exe_path() -> String {
        std::env::current_exe()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub fn get_states() -> HashMap<String, bool> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let mut out = HashMap::new();
        for group in GROUPS {
            let key = format!("Software\\Classes\\{}", progid(group));
            out.insert(group.to_string(), hkcu.open_subkey(&key).is_ok());
        }
        out
    }

    pub fn set_association(group: &str, enabled: bool) -> Result<bool, String> {
        let exts = group_exts(group);
        if exts.is_empty() {
            return Err(format!("Grupo de associacao desconhecido: {group}"));
        }
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let progid = progid(group);
        if enabled {
            let exe = exe_path();
            if exe.is_empty() {
                return Err("Nao foi possivel descobrir o caminho do executavel".into());
            }
            let classes = hkcu
                .open_subkey_with_flags("Software\\Classes", KEY_ALL_ACCESS | KEY_WOW64_64KEY)
                .or_else(|_| hkcu.open_subkey_with_flags("Software\\Classes", KEY_ALL_ACCESS))
                .map_err(|e| format!("Falha ao acessar HKCU\\Software\\Classes: {e}"))?;
            let def = classes
                .create_subkey(&progid)
                .map_err(|e| format!("Falha ao registrar {progid}: {e}"))?
                .0;
            def.set_value("", &format!("Imagem {}", group.to_uppercase()))
                .map_err(|e| format!("Falha ao gravar ProgID: {e}"))?;
            def.create_subkey("DefaultIcon")
                .and_then(|(k, _)| k.set_value("", &format!("\"{exe}\",0")))
                .map_err(|e| format!("Falha ao gravar icone: {e}"))?;
            def.create_subkey("shell\\open\\command")
                .and_then(|(k, _)| k.set_value("", &format!("\"{exe}\" \"%1\"")))
                .map_err(|e| format!("Falha ao gravar comando: {e}"))?;
            for ext in exts {
                let open_with = classes
                    .create_subkey(&format!(".{ext}\\OpenWithProgids"))
                    .map_err(|e| format!("Falha ao registrar .{ext}: {e}"))?
                    .0;
                open_with
                    .set_raw_value(
                        &progid,
                        &RegValue {
                            bytes: Vec::new(),
                            vtype: REG_NONE,
                        },
                    )
                    .map_err(|e| format!("Falha ao adicionar abre-com para .{ext}: {e}"))?;
            }
        } else {
            let classes = hkcu
                .open_subkey_with_flags("Software\\Classes", KEY_ALL_ACCESS | KEY_WOW64_64KEY)
                .or_else(|_| hkcu.open_subkey_with_flags("Software\\Classes", KEY_ALL_ACCESS))
                .map_err(|e| format!("Falha ao acessar HKCU\\Software\\Classes: {e}"))?;
            let _ = classes.delete_subkey_all(&progid);
            for ext in exts {
                if let Ok(open_with) = classes
                    .open_subkey_with_flags(&format!(".{ext}\\OpenWithProgids"), KEY_ALL_ACCESS)
                {
                    let _ = open_with.delete_value(&progid);
                }
            }
        }
        Ok(enabled)
    }

    pub fn avifnim_installed() -> bool {
        const ROOTS: &[(&str, &str)] = &[
            (
                "HKCU",
                "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
            ),
            (
                "HKLM",
                "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
            ),
            (
                "HKLM",
                "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
            ),
        ];
        for (root_name, path) in ROOTS {
            let root = if *root_name == "HKCU" {
                RegKey::predef(HKEY_CURRENT_USER)
            } else {
                RegKey::predef(HKEY_LOCAL_MACHINE)
            };
            if let Ok(uninstall) = root.open_subkey(path) {
                for name in uninstall.enum_keys().flatten() {
                    if let Ok(sub) = uninstall.open_subkey(&name) {
                        if let Ok(display) = sub.get_value::<String, _>("DisplayName") {
                            let d = display.to_lowercase();
                            if d.contains("avifnim") && !d.contains("viewer") {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        let candidates = [
            std::env::var("LOCALAPPDATA")
                .ok()
                .map(|p| format!("{p}\\Programs\\AVIFnim")),
            std::env::var("ProgramFiles")
                .ok()
                .map(|p| format!("{p}\\AVIFnim")),
        ];
        for dir in candidates.into_iter().flatten() {
            if std::path::Path::new(&dir).exists() {
                return true;
            }
        }
        false
    }
}

#[cfg(windows)]
pub use win::*;

#[cfg(not(windows))]
mod other {
    use super::*;

    pub fn get_states() -> HashMap<String, bool> {
        GROUPS.iter().map(|g| (g.to_string(), false)).collect()
    }

    pub fn set_association(_group: &str, _enabled: bool) -> Result<bool, String> {
        Err("Associacoes de arquivo sao gerenciadas pelo sistema operacional nesta plataforma".into())
    }

    pub fn avifnim_installed() -> bool {
        false
    }
}

#[cfg(not(windows))]
pub use other::*;

#[cfg(all(test, windows))]
mod tests {
    use super::*;

    #[test]
    fn association_roundtrip_avif() {
        // Registra e remove a associacao de teste do proprio usuario (HKCU),
        // garantindo que o estado inicial seja restaurado no final.
        let before = get_states();
        let had = *before.get("avif").unwrap_or(&false);
        let enabled = set_association("avif", true).expect("registrar avif");
        assert!(enabled);
        assert!(*get_states().get("avif").unwrap_or(&false));
        let disabled = set_association("avif", false).expect("remover avif");
        assert!(!disabled);
        assert!(!*get_states().get("avif").unwrap_or(&true));
        if had {
            let _ = set_association("avif", true);
        }
    }
}

