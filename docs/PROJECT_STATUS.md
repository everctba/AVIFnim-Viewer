# Status do projeto — AVIFnim Viewer v1.0

**Data:** 2026-11-13 · **Release:** [v0.1.1](https://github.com/everctba/AVIFnim-Viewer/releases/tag/v0.1.1)

> v0.1.1 adiciona o menu de contexto próprio sobre a imagem, duplo clique Fit/100%,
> prefetch padrão 3 e checkerboard desligado por padrão.

## Entregue

| Área | Estado | Evidência |
|---|---|---|
| App Tauri 2 + Rust + UI HTML/CSS/JS | ✅ | `cargo test` (24 testes), `cargo clippy -D warnings`, `cargo fmt --check` |
| AVIF animado/still, WebP, GIF | ✅ | screenshots em `docs/images`; animação AVIF confirmada por capturas |
| PPM/PNM P1–P6 (8/16-bit) → PNG | ✅ | 11 testes unitários + PPM 4K em 1,7 s |
| Navegação natural + prefetch LRU 128 MB | ✅ | testes + uso real |
| Zoom/pan/fit/100%/fullscreen/checkerboard/fundos | ✅ | screenshots e atalhos |
| Menu de contexto próprio + duplo clique Fit/100% | ✅ | screenshot `context-menu.png`; teste automatizado de duplo clique |
| Sequências PNG/JPG 30/60 fps (G7) | ✅ | 56,4 fps medidos; barra de sequência |
| i18n PT-BR/EN-US + tema claro/escuro (2 estados) | ✅ | screenshots dos dois temas |
| Associações opt-in + uninstall cleanup | ✅ | instalador custom + job `installer-test` no CI |
| Funil AVIFnim (modal 5ª visualização + banner) | ✅ | screenshot `promo.png` |
| Sobre com AVIF animado | ✅ | `about.png` |
| Windows (NSIS) | ✅ | instalador testado; asset no release |
| Linux (.deb) | ✅ | build + instalação + runtime Xvfb verdes no CI; asset no release |
| macOS (.dmg universal) | ✅ | build universal verde no CI; asset no release |
| Docs/licença | ✅ | README, LICENSE MIT, THIRD_PARTY_NOTICES, BUILD, QA, RELEASING, ADRs |
| Benchmarks | ✅ | startup 60 ms; AVIF 5,38 MB 258 ms; sequência 56,4 fps |
| Segurança (IPC + supply chain) | ✅ | allowlist `open_url`, validação de caminho, cargo audit/deny + SBOM no CI, Actions pinadas por SHA, [SECURITY.md](../SECURITY.md) |

## Pendências externas (bloqueios do owner/recursos)

| Item | Motivo |
|---|---|
| `avifnim-dd8` — Inno Setup vs NSIS | decisão do owner (NSIS já implementado e recomendado) |
| `avifnim-xqo` — notarização macOS | decisão/certificado Apple Developer |
| `avifnim-8hf` — auto-update | decisão do owner (recomendado sem na v1) |
| `avifnim-b53` — assinatura Windows | certificado de code signing |
| `avifnim-xhk` — logo/links oficiais | assets do owner |
| `avifnim-b7e` / `avifnim-2k7` — doações | QR codes PayPal/PIX do owner |
| `avifnim-wx5` — landing | publicar exige hospedagem + links/logo oficiais |

Nenhum desses itens impede o uso ou a distribuição da v1.0; todos são decisões/assets externos já
registrados no `bd` com recomendações nos ADRs (`docs/adr/`).

## Comandos úteis

```bash
bd ready                     # ver pendências externas
npx tauri build              # Windows: instalador NSIS
scripts\build.bat test       # testes Rust
scripts\test-installer.ps1   # opt-in/uninstall do instalador
```
