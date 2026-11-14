# AVIFnim Viewer 0.1.0 — instaladores

Artefatos gerados para a release **v0.1.0** (mesmos arquivos publicados em
https://github.com/everctba/AVIFnim-Viewer/releases/tag/v0.1.0).

| Arquivo | Plataforma | Tamanho | SHA-256 |
|---|---|---|---|
| `AVIFnim.Viewer_0.1.0_x64-setup.exe` | Windows 10/11 x64 (NSIS) | 1,32 MB | `3444c5e6…5ab4` |
| `AVIFnim.Viewer_0.1.0_amd64.deb` | Linux x64 (Ubuntu 24.04+/Mint, WebKitGTK 4.1) | 2,18 MB | `6172a779…a82f` |
| `AVIFnim.Viewer_0.1.0_universal.dmg` | macOS 13+ (Intel + Apple Silicon) | 6,06 MB | `d1b67c77…2a3e` |

Verificação: `sha256sum -c SHA256SUMS.txt` (Linux/macOS) ou
`Get-FileHash dist\arquivo -Algorithm SHA256` (Windows).

## Instalação

- **Windows:** execute o instalador. O assistente tem a página opcional de associações
  (tudo desmarcado) e permite escolher a pasta. Sem assinatura de código — o SmartScreen pode
  pedir confirmação.
- **Linux:** `sudo apt install ./AVIFnim.Viewer_0.1.0_amd64.deb`
- **macOS:** abra o `.dmg`, arraste o app para *Applications*. Sem notarização — na primeira vez,
  clique com o botão direito → *Abrir*.

Associações de arquivo **nunca** são alteradas sem ação do usuário (nem no instalador; opt-in pelas
configurações do app ou pela página do instalador).

## Observações

- Esta pasta é ignorada pelo git (binários). Ela é recriada com:
  `powershell -File scripts/collect-dist.ps1` (Windows) ou baixando da release.
- Hashes conferem com os assets da release v0.1.0.
