# QA — AVIFnim Viewer v1.0

Checklist de qualidade baseado nos critérios de aceite do PRD (§14) e nos requisitos funcionais.
Amostras em [`tests/assets`](../tests/assets); evidências visuais em [`docs/images`](images).

## Matriz de formatos

| Caso | Arquivo | Esperado | Status |
|---|---|---|---|
| AVIF still | `still.avif` | abre, exibe 96×64, badge AVIF | ✅ testado |
| AVIF animado | — | toca sozinho em loop | ⏳ pendente de amostra animada (sem encoder AVIF no ambiente; gerar com o AVIFnim) |
| WebP animado | `anim.webp` | toca em loop, banner contextual | ✅ testado ([viewer.png](images/viewer.png)) |
| GIF | `anim.gif` | toca em loop, banner contextual | ✅ testado |
| PPM binário P6 | `p6.ppm` | decodifica em Rust → PNG, exibe | ✅ testado + teste unitário |
| PPM ASCII P3 | `p3.ppm` | decodifica em Rust → PNG, exibe | ✅ testado + teste unitário |
| PNM P1/P2/P4/P5 | gerado nos testes | decodifica corretamente | ✅ 4 testes unitários |
| PNM 16-bit | gerado nos testes | converte para 8-bit e exibe | ✅ teste unitário |
| PPM malformado | `broken.ppm` | mensagem amigável, sem crash | ✅ testado (screenshot) |
| Sequência PNG | `seq/frame_0001..0012.png` | detecta 12 frames, loop 60 fps | ✅ testado ([sequence.png](images/sequence.png)) |
| Nomes unicode/espaços | `coração ção — teste áéí.avif` | abre e exibe o título corretamente | ✅ testado |
| Loop count 1x | — | toca uma vez e para | ⏳ manual (limitação documentada) |

## Navegação e UI

| Caso | Esperado | Status |
|---|---|---|
| Setas ← → e botões prev/next | troca de arquivo com ordenação natural | ✅ testado |
| Zoom +/-, fit, 100% | não distorce, label atualiza | ✅ testado |
| Fullscreen (F) | entra/sai da tela cheia | ✅ implementado |
| Checkerboard / fundos | alternam e persistem | ✅ implementado |
| Painel Info (I) | mostra nome/formato/dimensões/tamanho/caminho | ✅ testado |
| Configurações (S) | abre, persiste tema/idioma/prefetch/fundo/FPS | ✅ testado ([settings.png](images/settings.png)) |
| Sobre (F1) | autor, links, limitações, banner do AVIFnim | ✅ testado ([about.png](images/about.png); AVIF animado do banner pendente de asset) |
| Funil (modal após 5ª visualização) | aparece uma vez, dismiss persiste | ✅ testado ([promo.png](images/promo.png)) |
| Banner GIF/WebP | economia estimada + CTA dispensável | ✅ testado |
| Instância única + abrir por argumento | segunda instância envia arquivo para a janela existente | ✅ testado |
| Drag & drop | arquivo abre ao soltar na janela | ✅ implementado (evento Tauri) |

## Segurança / robustez

| Caso | Esperado | Status |
|---|---|---|
| Header PNM com dimensões absurdas | recusa com cap (100 MPix) sem OOM | ✅ teste unitário |
| PNM com magic desconhecido | erro amigável | ✅ teste unitário |
| PNM 0×0 | erro amigável | ✅ teste unitário |
| Arquivo > 256 MB | recusa antes de ler | ✅ implementado |
| Arquivo corrompido (AVIF/WebP/GIF) | erro amigável, app continua utilizável | ⏳ manual |
| CSP restritiva / sem shell / sem conteúdo remoto | configuração do Tauri | ✅ revisão de código |
| Sem telemetria / sem upload | somente leitura local | ✅ revisão de código |
| Associações opt-in (registro HKCU) | só registra após o usuário marcar; remove ao desmarcar | ✅ teste unitário de roundtrip |

## Desempenho (benchmark)

| Métrica | Meta | Status |
|---|---|---|
| Abrir AVIF 5 MB (SSD) | < 300 ms | ⏳ pendente (issue `avifnim-v0w`) |
| Reprodução 1080p | 60 fps | ⏳ pendente |
| Startup | < 1,0 s | ⏳ pendente |
| Memória do prefetch | ≤ teto configurado | ✅ cache LRU com cap + testes unitários |

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
