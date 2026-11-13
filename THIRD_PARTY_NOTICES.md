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

## Notas

- O decode de AVIF/WebP/GIF é feito pelo **webview do sistema** — nenhum decoder adicional
  proprietário é embarcado.
- Se no futuro entrar um decoder próprio ou FFmpeg, os avisos e a política de licenciamento
  deverão ser atualizados (mesma estratégia do AVIFnim: processo separado + notices).
- Lista completa de dependências Rust: `src-tauri/Cargo.toml` e `Cargo.lock`.
