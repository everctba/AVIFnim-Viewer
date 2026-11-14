# Release — AVIFnim Viewer

Passo a passo para publicar uma versão (usado na v0.1.0).

## 1. Preparação

```bash
cd src-tauri && cargo test        # 24 testes
npm install
scripts\build.bat build           # release local (Windows)
npx tauri build                   # instalador NSIS em target/release/bundle/nsis
```

Atualize `CHANGELOG.md`, a versão no `package.json`/`src-tauri/Cargo.toml`/`tauri.conf.json`
(se mudar) e faça o commit/push em `master`.

## 2. Artefatos multiplataforma (GitHub Actions)

Dispare o workflow **Platforms** (Actions → Platforms → Run workflow) ou via API:

```bash
curl -X POST -H "Authorization: Bearer $GITHUB_TOKEN" \
  -H "Accept: application/vnd.github+json" \
  https://api.github.com/repos/everctba/AVIFnim-Viewer/actions/workflows/platforms.yml/dispatches \
  -d '{"ref":"master"}'
```

- `linux-deb`: `.deb` (ubuntu-22.04, WebKitGTK 4.1)
- `macos-dmg`: `.dmg` universal (macos-13, Intel + Apple Silicon)

Baixe os artefatos em Actions → run → Artifacts (ou via API `/actions/artifacts/{id}/zip`).

## 3. Publicação

Crie a tag e a release (exemplo via API):

```bash
# tag anotada apontando para o commit final
git tag -a v0.1.0 -m "AVIFnim Viewer 0.1.0"
git push origin v0.1.0

# release pública
curl -X POST -H "Authorization: Bearer $GITHUB_TOKEN" \
  -H "Accept: application/vnd.github+json" \
  https://api.github.com/repos/everctba/AVIFnim-Viewer/releases \
  -d '{"tag_name":"v0.1.0","name":"AVIFnim Viewer 0.1.0","body":"...","draft":false}'
```

Anexe os assets (instalador Windows, `.deb`, `.dmg`):

```bash
curl -X POST -H "Authorization: Bearer $GITHUB_TOKEN" \
  -H "Content-Type: application/octet-stream" \
  --data-binary "@AVIFnim Viewer_0.1.0_x64-setup.exe" \
  "https://uploads.github.com/repos/everctba/AVIFnim-Viewer/releases/$RELEASE_ID/assets?name=AVIFnim.Viewer_0.1.0_x64-setup.exe"
```

> O GitHub normaliza espaços do nome do asset para pontos.

## 4. Checklist pós-release

- [ ] `curl .../releases/tags/v0.1.0` mostra os 3 assets (`.exe`, `.deb`, `.dmg`)
- [ ] Instalador Windows instala, abre um AVIF animado e desinstala limpo
- [ ] `.deb` instala em Ubuntu 24.04 e abre; `.dmg` abre no macOS 13+
- [ ] Landing (`landing/index.html`) atualizada com os links
- [ ] Issues no `bd` fechadas/atualizadas; `bd dolt push`
- [ ] Anúncio (owner)

## Pendências conhecidas da v0.1.0

- Instalador Windows **sem assinatura de código** (issue `avifnim-dd8`).
- macOS **sem notarização** (issue `avifnim-xqo`); instruir "Abrir mesmo assim".
- Builds multiplataforma gerados no CI; execução real em macOS/Linux ainda não testada em hardware.
