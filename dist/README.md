# AVIFnim Viewer — instaladores locais

Pasta de artefatos locais (binários **não** versionados; README/checksums são).

## Build atual — 0.1.2 (não publicado; portátil Windows em `d6b070f`, Linux/macOS em `fb959d6`)

| Arquivo | Plataforma | Tamanho | SHA-256 |
|---|---|---|---|
| `AVIFnim.Viewer_0.1.2_x64-setup.exe` | Windows 10/11 x64 (NSIS) | 3,75 MB | `73765fa3…c029ee` |
| `AVIFnim.Viewer_0.1.2_x64-unpacked/AVIFnim Viewer.exe` | Windows x64 **portátil** (sem instalar) | 6,43 MB | `3af80d0b…7b60f1` |
| `AVIFnim.Viewer_0.1.2_x64.msix` | Windows x64 — **Microsoft Store** (sem assinatura) | 4,29 MB | `0f7137b2…855f47` |
| `avifnim-viewer_0.1.2_amd64.deb` | Linux x64 (Ubuntu 22.04+/Mint) | 4,19 MB | `c72fd2e9…f5fc7c` |
| `AVIFnim.Viewer_0.1.2_universal.dmg` | macOS 13+ (Intel + Apple Silicon) | 12,68 MB | `7b55eaeb…11cb5b` |

> Portátil Windows em `d6b070f` (badge do formato colado ao nome); instalador/MSIX em `bfdf2e7`.
> `.msix` é um alvo **separado** para a Microsoft Store (não substitui o instalador; não instale os
> dois juntos). Linux/macOS em `fb959d6`: AVIF fallback com limite por PNG comprimido + DMG/MIME.

**Portátil (unpacked):** abra `AVIFnim.Viewer_0.1.2_x64-unpacked/AVIFnim Viewer.exe` diretamente —
não instala, não cria atalhos e não registra associações. Requer o WebView2 Runtime (Windows 10/11
atualizado já possui). Instruções em `LEIA-ME.txt` dentro da pasta.

Linux `.deb` e macOS `.dmg` foram gerados pelo workflow **Platforms** (commit `be06753`); o `.deb`
passou no smoke test headless (Xvfb + WebKitGTK). **Ainda não publicados** no GitHub.

> Esta build inclui o **fallback AVIF nativo** (`avif-rust`): em Linux cujo WebKitGTK não decodifica
> AVIF (still ou animado), o app decodifica em Rust e toca os frames com as durações corretas.
> No Linux, o `postinst` do `.deb` cria automaticamente o atalho na **área de trabalho** do usuário
> que usou `sudo` (e o `postrm` remove no uninstall).

> O pacote Debian agora se chama **`avifnim-viewer`** (antes `avi-fnim-viewer`, nome derivado do
> productName pelo Tauri). Se você tem o antigo instalado, remova antes:
> `sudo apt remove avi-fnim-viewer`.

## Release publicada — v0.1.1

Windows, Linux e macOS de 0.1.1 estão em
https://github.com/everctba/AVIFnim-Viewer/releases/tag/v0.1.1

## Instalação e verificação

- **Windows:** execute o instalador (página de associações desmarcada; pasta configurável).
- **Linux:** `sudo apt install ./avifnim-viewer_0.1.2_amd64.deb`
- **macOS:** abra o `.dmg` e arraste o app; sem notarização — botão direito → *Abrir* na 1ª vez.
- Checksums: `sha256sum -c SHA256SUMS.txt` ou `Get-FileHash` no PowerShell.

Recriar esta pasta: `powershell -File scripts/collect-dist.ps1` (baixa da release) + copiar os
artefatos dos builds locais/CI (`src-tauri/target/release/bundle`, artifacts do workflow Platforms).
