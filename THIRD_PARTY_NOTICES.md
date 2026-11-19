# Avisos de terceiros — AVIFnim Viewer

O AVIFnim Viewer é distribuído sob a licença **MIT**. Abaixo estão os componentes de terceiros
usados no binário e suas licenças. O binário **não inclui FFmpeg nem qualquer componente GPL**
na v1.

## Rust / Tauri

| Componente | Licença | Uso |
|---|---|---|
| [Tauri](https://tauri.app/) 2 | MIT / Apache-2.0 | framework do app (janela, IPC, webview) |
| [Wry](https://github.com/tauri-apps/wry) | MIT / Apache-2.0 | integração com o webview do sistema |
| [Tao](https://github.com/tauri-apps/tao) | MIT / Apache-2.0 | janelas e event loop |
| [serde](https://serde.rs/) / serde_json | MIT / Apache-2.0 | serialização |
| [base64](https://crates.io/crates/base64) | MIT / Apache-2.0 | encoding das imagens para o webview |
| [image](https://crates.io/crates/image) | MIT / Apache-2.0 | decode PNM/PPM (P1–P6) e encode PNG |
| [avif-rust](https://crates.io/crates/avif-rust) | MIT | fallback nativo de decode AVIF (still + animado) quando o webview não suporta |
| [avif-image-buffer](https://crates.io/crates/avif-image-buffer) | MIT | buffers RGBA do fallback AVIF |
| [bin-rs](https://crates.io/crates/bin-rs) | MIT | leitura binária do container AVIF |
| [rfd](https://crates.io/crates/rfd) | MIT | diálogo nativo de abrir arquivo |
| [open](https://crates.io/crates/open) | MIT | abrir links no navegador/SO |
| [winreg](https://crates.io/crates/winreg) | MIT | associações opt-in no registro (Windows) |
| [tauri-plugin-single-instance](https://github.com/tauri-apps/plugins-workspace) | MIT / Apache-2.0 | instância única |

## Componentes do sistema operacional

| Componente | Licença / termos | Plataforma |
|---|---|---|
| WebView2 (Chromium/Edge) | termos Microsoft | Windows |
| WKWebView / WebKit | componentes do sistema | macOS |
| WebKitGTK 4.1 | LGPL (biblioteca do sistema, não embarcada) | Linux |

## Fontes

| Componente | Licença | Uso |
|---|---|---|
| [Inter](https://rsms.me/inter/) (variable, woff2) | **SIL Open Font License 1.1** | tipografia da interface, embarcada em `src/assets/fonts/` (`LICENSE.txt`) |

## Notas

- AVIF/WebP/GIF são decodificados pelo **webview do sistema**; quando o webview não tem decoder
  AVIF, o app usa **avif-rust (MIT, Rust puro)** como fallback — sem FFmpeg/GPL.
- Novos decoders/formatos devem atualizar esta lista e a política de licenciamento (mesma estratégia
  do AVIFnim: processo separado + notices).
- Lista completa de dependências Rust: `src-tauri/Cargo.toml` e `Cargo.lock`.
