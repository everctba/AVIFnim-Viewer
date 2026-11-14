# AVIFnim Viewer — instaladores locais

Pasta de artefatos locais (binários **não** versionados; README/checksums são).

## Build de teste atual — Windows 0.1.2 (ícone novo)

| Arquivo | Plataforma | Tamanho | SHA-256 |
|---|---|---|---|
| `AVIFnim.Viewer_0.1.2_x64-setup.exe` | Windows 10/11 x64 (NSIS) | 1,34 MB | `599e2d65…f26d` |
| `AVIFnim.Viewer_0.1.2_x64-unpacked/AVIFnim Viewer.exe` | Windows x64 **portátil** (sem instalar) | 3,44 MB | `ea1a7a0b…f74` |

**Portátil (unpacked):** abra `AVIFnim.Viewer_0.1.2_x64-unpacked/AVIFnim Viewer.exe` diretamente —
não instala, não cria atalhos e não registra associações. Requer o WebView2 Runtime (Windows 10/11
atualizado já possui). Instruções em `LEIA-ME.txt` dentro da pasta.

Este build **ainda não foi publicado** no GitHub: é para você validar o ícone novo
(instalador, título da janela, barra de tarefas) e o menu de contexto/duplo clique.
Quando o visual estiver aprovado, será feito o build final de **todas** as plataformas
e a publicação da versão correspondente.

## Release atual — v0.1.1 (Linux/macOS)

| Arquivo | Plataforma | Tamanho | SHA-256 |
|---|---|---|---|
| `AVIFnim.Viewer_0.1.1_amd64.deb` | Linux x64 (Ubuntu 24.04+/Mint) | 2,18 MB | `c1ed104c…b5f92` |
| `AVIFnim.Viewer_0.1.1_universal.dmg` | macOS 13+ (Intel + Apple Silicon) | 6,07 MB | `fc6857cf…86d10` |

Publicados em https://github.com/everctba/AVIFnim-Viewer/releases/tag/v0.1.1

## Instalação e verificação

- **Windows:** execute o instalador (página de associações desmarcada; pasta configurável).
- **Linux:** `sudo apt install ./AVIFnim.Viewer_0.1.1_amd64.deb`
- **macOS:** abra o `.dmg` e arraste o app; sem notarização — botão direito → *Abrir* na 1ª vez.
- Checksums: `sha256sum -c SHA256SUMS.txt` ou `Get-FileHash` no PowerShell.

Recriar esta pasta: `powershell -File scripts/collect-dist.ps1` (baixa da release) + copiar o
instalador Windows do build local (`src-tauri/target/release/bundle/nsis`).
