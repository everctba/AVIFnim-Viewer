# AVIFnim Viewer 0.1.1 — instaladores

Artefatos gerados para a release **v0.1.1** (mesmos arquivos publicados em
https://github.com/everctba/AVIFnim-Viewer/releases/tag/v0.1.1).

| Arquivo | Plataforma | Tamanho | SHA-256 |
|---|---|---|---|
| `AVIFnim.Viewer_0.1.1_x64-setup.exe` | Windows 10/11 x64 (NSIS) | 1,32 MB | `d20b5cf4…8764` |
| `AVIFnim.Viewer_0.1.1_amd64.deb` | Linux x64 (Ubuntu 24.04+/Mint, WebKitGTK 4.1) | 2,18 MB | `c1ed104c…b5f92` |
| `AVIFnim.Viewer_0.1.1_universal.dmg` | macOS 13+ (Intel + Apple Silicon) | 6,07 MB | `fc6857cf…86d10` |

Verificação: `sha256sum -c SHA256SUMS.txt` (Linux/macOS) ou
`Get-FileHash dist\arquivo -Algorithm SHA256` (Windows).

## Novidades da 0.1.1

- Menu de contexto próprio sobre a imagem (substitui o menu nativo do webview).
- Duplo clique alterna Fit ↔ 100%.
- Prefetch padrão 3 (antes 2) e checkerboard desligado por padrão (configurável).

## Instalação

- **Windows:** execute o instalador. O assistente tem a página opcional de associações
  (tudo desmarcado) e permite escolher a pasta. Sem assinatura de código — o SmartScreen pode
  pedir confirmação.
- **Linux:** `sudo apt install ./AVIFnim.Viewer_0.1.1_amd64.deb`
- **macOS:** abra o `.dmg`, arraste o app para *Applications*. Sem notarização — na primeira vez,
  clique com o botão direito → *Abrir*.

Associações de arquivo **nunca** são alteradas sem ação do usuário (nem no instalador; opt-in pelas
configurações do app ou pela página do instalador).

## Observações

- Esta pasta é ignorada pelo git (binários). Ela é recriada com:
  `powershell -File scripts/collect-dist.ps1` (Windows) ou baixando da release.
- Hashes conferem com os assets da release v0.1.1.
