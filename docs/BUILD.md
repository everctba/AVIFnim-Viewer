# Build — AVIFnim Viewer

## Windows (x64 / MSVC)

Pré-requisitos:

1. **Rust** (`rustup` com toolchain `stable-x86_64-pc-windows-msvc`).
2. **Visual Studio Build Tools 2019/2022** com a workload *Desktop development with C++*
   (MSVC + Windows SDK).
3. **Node.js 18+** e `npm install` na raiz do repositório (CLI do Tauri).
4. **WebView2 Runtime** (Windows 10/11 atualizado já tem).

### Ambiente MSVC

O `cargo` precisa do `link.exe` e das variáveis `LIB`/`INCLUDE` do MSVC. Fora do
“x64 Native Tools Command Prompt”, use o script deste repositório:

```powershell
# build de desenvolvimento
scripts\build.ps1 check
scripts\build.ps1 run
scripts\build.ps1 test

# app + instalador
scripts\build.ps1 build
```

O script chama `vcvars64.bat` (localizado via `vswhere`) e repassa os argumentos ao `cargo`.

### Comandos diretos (dentro do Developer Command Prompt)

```bash
npm install
npx tauri build            # instalador NSIS + exe
npx tauri dev              # desenvolvimento
cd src-tauri && cargo test # testes unitários
```

Artefatos:

- `src-tauri/target/release/avifnim-viewer.exe`
- `src-tauri/target/release/bundle/nsis/AVIFnim Viewer_*_x64-setup.exe`

> Assinatura de código (certificado) e notarização (macOS) dependem de decisões registradas
> nas issues `avifnim-dd8`, `avifnim-xqo` e `avifnim-8hf`.

## macOS (13+)

```bash
npm install
npx tauri build --target universal-apple-darwin   # Intel + Apple Silicon
```

Requer Xcode Command Line Tools. Notarização/assinatura conforme decisão (`avifnim-xqo`).

## Linux (x64)

```bash
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
npm install
npx tauri build --bundles deb
```

WebKitGTK 4.1 com suporte a AVIF animado é o principal risco — ver spike `avifnim-6zp`.

## Builds de CI (GitHub Actions)

Além do build local, o repositório tem workflows para gerar os artefatos multiplataforma sem
precisar de macOS/Linux na máquina:

| Workflow | Runner | Artefato |
|---|---|---|
| `.github/workflows/platforms.yml` → `linux-deb` | `ubuntu-22.04` | `src-tauri/target/release/bundle/deb/*.deb` |
| `.github/workflows/platforms.yml` → `macos-dmg` | `macos-14` (universal Intel + ARM) | `src-tauri/target/universal-apple-darwin/release/bundle/dmg/*.dmg` |
| `.github/workflows/ci.yml` | `windows-latest` | testes + clippy + build debug |
| `.github/workflows/release.yml` | `windows-latest` | release draft com instalador NSIS |

Para disparar manualmente (via API ou GitHub UI em Actions → Platforms → Run workflow):

```bash
curl -X POST \
  -H "Authorization: Bearer $GITHUB_TOKEN" \
  -H "Accept: application/vnd.github+json" \
  https://api.github.com/repos/everctba/AVIFnim-Viewer/actions/workflows/platforms.yml/dispatches \
  -d '{"ref":"master"}'
```

Os artefatos ficam disponíveis em Actions → run → Artifacts. O `.deb` pode ser inspecionado com
`dpkg-deb -I arquivo.deb` e instalado com `sudo apt install ./arquivo.deb`; o `.dmg` é aberto no macOS.

### Instalador Windows (NSIS customizado)

O instalador usa template próprio (`src-tauri/windows/installer.nsi`, baseado no template oficial do
Tauri 2.12.4) + hooks (`src-tauri/windows/nsis-hooks.nsh`):

- **Página de associações opt-in**: 4 checkboxes (AVIF, WebP, GIF, PPM/PNM) **desmarcados por padrão**.
- **Escolha da pasta** na página de diretório (com fallback robusto para a pasta padrão).
- **Uninstall** remove as associações criadas em `HKCU\Software\Classes`.
- Opt-in silencioso para automação/CI: `/S /ASSOC=avif,webp,gif,pnm` (subconjunto) ou `/S /ASSOC` (todos).
  Sem a flag, nada é registrado.

```bash
npx tauri build --bundles nsis
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/test-installer.ps1
```

> Ao atualizar a versão do Tauri CLI, revise o template customizado contra o template oficial da
> mesma versão (os placeholders `{{...}}` mudam entre versões).

## Testes

```bash
cd src-tauri
cargo test
```

Cobertura atual: decode PNM (P3/P6, headers inválidos, caps de pixels), cache LRU do prefetch
e ordenação natural de arquivos. Amostras de QA em `tests/assets/`.
