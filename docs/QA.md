# QA — AVIFnim Viewer v1.0

Checklist de qualidade baseado nos critérios de aceite do PRD (§14) e nos requisitos funcionais.
Amostras em [`tests/assets`](../tests/assets); evidências visuais em [`docs/images`](images).

## Matriz de formatos

| Caso | Arquivo | Esperado | Status |
|---|---|---|---|
| AVIF still | `still.avif` | abre, exibe 96×64, badge AVIF | ✅ testado |
| AVIF animado | `anim.avif` (8 frames) | toca sozinho em loop | ✅ testado ([viewer-avif.png](images/viewer-avif.png)); animação confirmada por 2 capturas com hashes diferentes |
| WebP animado | `anim.webp` | toca em loop, banner contextual | ✅ testado ([viewer.png](images/viewer.png)) |
| GIF | `anim.gif` | toca em loop, banner contextual | ✅ testado |
| PPM binário P6 | `p6.ppm` | decodifica em Rust → PNG, exibe | ✅ testado + teste unitário |
| PPM ASCII P3 | `p3.ppm` | decodifica em Rust → PNG, exibe | ✅ testado + teste unitário |
| PNM P1/P2/P4/P5 | gerado nos testes | decodifica corretamente | ✅ 4 testes unitários |
| PNM 16-bit | gerado nos testes | converte para 8-bit e exibe | ✅ teste unitário |
| PPM malformado | `broken.ppm` | mensagem amigável, sem crash | ✅ testado (screenshot) |
| Sequência PNG | `seq/frame_0001..0012.png` | detecta 12 frames, loop 60 fps | ✅ testado ([sequence.png](images/sequence.png)) |
| Nomes unicode/espaços | `coração ção — teste áéí.avif` | abre e exibe o título corretamente | ✅ testado |
| Loop count 1x | — | toca uma vez e para | ✅ testado: `loop1.gif` — capturas em 1 s e 5 s idênticas (parou após tocar uma vez) |

## Navegação e UI

| Caso | Esperado | Status |
|---|---|---|
| Setas ← → e botões prev/next | troca de arquivo com ordenação natural | ✅ testado |
| Zoom +/-, fit, 100% | não distorce, label atualiza | ✅ testado |
| Duplo clique esquerdo | alterna Fit ↔ 100% | ✅ testado (capturas: label 100% e Fit) |
| Menu de contexto (botão direito) | substitui menu nativo; ações e fechamento (Esc/clique fora) | ✅ testado ([context-menu.png](images/context-menu.png)) |
| Fullscreen (F) | entra/sai da tela cheia | ✅ implementado |
| Checkerboard / fundos | alternam e persistem | ✅ implementado |
| Painel Info (I) | mostra nome/formato/dimensões/tamanho/caminho | ✅ testado |
| Configurações (S) | abre, persiste tema/idioma/prefetch/fundo/FPS | ✅ testado ([settings.png](images/settings.png)) |
| Sobre (F1) | autor, links, limitações, banner do AVIFnim com AVIF animado | ✅ testado ([about.png](images/about.png)) |
| Funil (modal após 5ª visualização) | aparece uma vez, dismiss persiste | ✅ testado ([promo.png](images/promo.png)) |
| Banner GIF/WebP | economia estimada + CTA dispensável | ✅ testado |
| Instância única + abrir por argumento | segunda instância envia arquivo para a janela existente | ✅ testado |
| Drag & drop | arquivo abre ao soltar na janela | ✅ implementado (evento Tauri) |
| Idioma EN-US | troca sem reiniciar, textos completos | ✅ testado ([viewer-light-en.png](images/viewer-light-en.png)) |
| Tema claro | UI clara + title bar nativa clara | ✅ testado (mesma captura) |

## Segurança / robustez

| Caso | Esperado | Status |
|---|---|---|
| Header PNM com dimensões absurdas | recusa com cap (100 MPix) sem OOM | ✅ teste unitário |
| PNM com magic desconhecido | erro amigável | ✅ teste unitário |
| PNM 0×0 | erro amigável | ✅ teste unitário |
| Arquivo > 256 MB | recusa antes de ler | ✅ implementado |
| Arquivo corrompido (AVIF/WebP/GIF) | erro amigável, app continua utilizável | ✅ testado: `corrupt-anim.avif` (e webp/gif truncados) exibem "Não foi possível decodificar este arquivo" com "Abrir outro", sem crash (validado também no build instalado) |
| CSP restritiva / sem shell / sem conteúdo remoto | configuração do Tauri | ✅ revisão de código |
| Sem telemetria / sem upload | somente leitura local | ✅ revisão de código |
| Associações opt-in (registro HKCU) | só registra após o usuário marcar; remove ao desmarcar | ✅ teste unitário de roundtrip |
| Instalador: página de associações opt-in | checkboxes desmarcados por padrão; seleção grava ProgIDs/OpenWithProgids | ✅ testado: página renderizada ([installer-associations.png](images/installer-associations.png)); `/ASSOC=avif` grava só AVIF; `/ASSOC` grava todos; sem flag não grava nada |
| Instalador: escolha da pasta | página de diretório com pasta padrão preenchida e "Browse…" | ✅ corrigido fallback de `$INSTDIR` e validado no fluxo GUI |
| Uninstall remove associações | hook NSIS limpa ProgIDs e OpenWithProgids | ✅ testado: 3 cenários de install/uninstall automatizados (`scripts/test-installer.ps1`) |
| Instalador no CI | build + testes de opt-in/uninstall automáticos | ✅ job `installer-test` no `ci.yml` |

## Plataformas

| Plataforma | Evidência |
|---|---|
| **Windows 10/11 x64** | build local + instalador NSIS testado (instala/roda/desinstala); release v0.1.0 publicado |
| **Linux x64 (.deb)** | build + **runtime validados no GitHub Actions**: job `linux-deb` (`ubuntu-22.04`, WebKitGTK 4.1) e job `linux-runtime` instala o `.deb` com `apt` e roda `avifnim-viewer` por 15 s sob Xvfb **sem crash** (SUCCESS). `.deb` validado: magic `!<arch>`, control `Package: avi-fnim-viewer`, `Version: 0.1.0`, `Architecture: amd64`, `Depends: libwebkit2gtk-4.1-0, libgtk-3-0`, data com `usr/bin/avifnim-viewer`, `.desktop` e ícones. Publicado no release v0.1.0. |
| **macOS 13+ (.dmg universal)** | build **green** no GitHub Actions (`macos-latest`, targets Intel + Apple Silicon): `.dmg` universal gerado (6,06 MB), trailer UDIF `koly` validado, publicado no release v0.1.0 (sha256 `d1b67c77…`). Compila o código macOS-específico (`RunEvent::Opened`, UTIs). Execução real em hardware/VM macOS e notarização: pendentes (`avifnim-xqo`). |

## Desempenho (benchmark)

| Métrica | Meta | Medido |
|---|---|---|
| Startup (janela criada) | < 1,0 s | ✅ **30–78 ms** (avg 60 ms, 5 runs, release, Windows 10) |
| Abrir AVIF 5,38 MB (2400×1350, SSD) | < 300 ms | ✅ **258 ms** (até o primeiro paint) |
| Abrir AVIF 1080p 3,44 MB | — | ✅ **180 ms** (primeiro paint) |
| Abrir AVIF animado pequeno | — | ✅ 32 ms (primeiro paint) |
| Abrir PPM 4K (4096×2160, 26,5 MB) | — | ✅ 1.698 ms (decode Rust → PNG → base64 → paint; cap de pixels ok) |
| Reprodução de sequência | 60 fps | ✅ **56,4 fps** medidos (12 frames PNG, janela de 2 s) |
| Animação AVIF/WebP/GIF em 1080p | 60 fps | ⏳ controlada pelo WebView2; sem API JS para contar frames de `<img>` animado (medir por captura/gravação de tela fica como follow-up) |
| Memória do prefetch | ≤ teto configurado | ✅ cache LRU com cap + testes unitários |

### Modo de benchmark (reproduzível)

O app tem um modo de benchmark ativado por variável de ambiente:

```powershell
$env:AVIFNIM_BENCH = "1"
& "src-tauri\target\release\avifnim-viewer.exe" "tests\assets\big-5mb.avif"
# resultados em %APPDATA%\com.avifnim.viewer\bench.log (JSON por linha)
```

Eventos emitidos: `first_paint` (ms do início da abertura até o frame pintado) e
`sequence_fps` (frames exibidos em ~2 s numa sequência).

- Startup: `powershell -File scripts\bench-startup.ps1 -Runs 5`
- Amostras: `python scripts/make-test-assets.py` (gera `big-1080p.avif` e `big-5mb.avif`)

## Testes automatizados

```bash
cd src-tauri && cargo test
```

Cobertura: decode PNM P1–P6 (8/16-bit), headers inválidos, caps de pixels, cache LRU do prefetch,
ordenação natural, parsing de argumentos, roundtrip de configurações (incluindo BOM) e
roundtrip de associação no registro do Windows.

## Limitações conhecidas (documentadas no app)

- Loop count 1x toca uma vez e para.
- FPS efetivo depende do webview do sistema.
- PPM/PNM é still e assume sRGB (sem perfil de cor).
- Instalador NSIS não registra associações automaticamente (opt-in só pelas configurações do app).
- Assinatura de código/notarização dependem de decisões (`avifnim-dd8`, `avifnim-xqo`).
