# PRD — AVIFnim Viewer

**Produto:** AVIFnim Viewer (gratuito e open source)
**Versão do documento:** 1.0 — draft para aprovação
**Data:** 13/11/2026
**Owner:** Everson Mayer · mayerdesigner@gmail.com
**Licença do software:** MIT (o AVIFnim pago continua proprietário)
**Repositório:** projeto separado (novo repositório), sem compartilhar código com o AVIFnim

> **Nota:** este PRD descreve um produto independente. Ele pode viver neste `docs/` apenas como
> rascunho; o desenvolvimento deve acontecer em um repositório próprio para não misturar com o
> AVIFnim (conversor pago). A única ligação entre os dois é a marca, o link de funil e o botão
> “Converter com AVIFnim”.

---

## 1. Sumário executivo

O **AVIFnim Viewer** é um visualizador gratuito, leve e multiplataforma para **imagens animadas**
(AVIF, WebP e GIF) e para **stills/sequências PPM/PNM**. Ele resolve um problema real: os
visualizadores nativos dos sistemas operacionais abrem AVIF, mas em geral **não animam**; no
Windows, o Photos mostra apenas o primeiro frame; no Linux, o suporte varia; e PPM não é suportado
por webview nenhum.

O produto é construído em **Tauri 2 + Rust**, usando o webview do sistema para os formatos animados
e um decodificador PNM próprio (Rust puro) para PPM/PNM. O escopo é deliberadamente mínimo:
**abrir, tocar em loop e navegar** — sem pausa, sem seek, sem edição. O app é gratuito, sem
telemetria, com associações de arquivo **opt-in e desmarcadas por padrão**, e um funil discreto para
o AVIFnim pago (modal único + banner contextual em GIF/WebP).

**Objetivo estratégico:** ser a porta de entrada para o AVIFnim. Quem baixa o viewer descobre que
abre AVIF animado.

---

## 2. Contexto e problema

### 2.1 O problema

AVIF animado é uma sequência de imagens dentro de um container HEIF. O pipeline de imagem dos
sistemas operacionais é “still-first”, então:

| Plataforma | Visualizador nativo | AVIF animado |
|---|---|---|
| Windows 10/11 | Fotos (com AV1 Video Extension) | abre, mas mostra o primeiro frame |
| Windows 10/11 | Photo Viewer antigo | não abre / não anima |
| macOS 13+ | Quick Look | anima, com teto de FPS |
| macOS 13+ | Preview | normalmente primeiro frame |
| Linux (GNOME) | Loupe/eog | abre; animação inconsistente |
| Linux (KDE) | Gwenview | anima |
| Navegadores | Chrome/Edge/Firefox/Safari | animam |

Além disso, **PPM/PNM** (comum em pipelines de render/VFX) não é exibido por webviews nem por
visualizadores comuns. Quem trabalha com sequências PPM precisa de uma ferramenta específica.

### 2.2 Por que agora

- O AVIFnim já exporta AVIF animado; o usuário precisa conseguir **ver** o resultado.
- O Windows 10 saiu de suporte (out/2025), mas ainda é usado; o Windows 11 também não anima no Fotos.
- O público de render/VFX exporta PPM e precisa inspecionar frames antes de converter.
- Um viewer grátis é o canal de aquisição mais barato para o AVIFnim pago.

### 2.3 Alternativas existentes

| Alternativa | Força | Fraqueza |
|---|---|---|
| XnView MP | toca AVIF animado, muitos formatos | pesado, UI datada, sem foco em AVIFnim |
| Gwenview (KDE) | toca AVIF animado | só KDE/Linux |
| Navegador | toca tudo | não é “duplo clique”, sem associação |
| Quick Look | integrado ao macOS | teto de FPS, sem PPM, sem funil |
| Fotos (Windows) | integrado | não anima |

**Diferencial do AVIFnim Viewer:** leve, gratuito, MIT, abre AVIF/WebP/GIF **e** PPM/PNM, navega
sequências com prefetch bidirecional, associações opcionais e integração com o AVIFnim pago.

---

## 3. Objetivos e não-objetivos

### 3.1 Objetivos

- **G1** — Abrir e tocar AVIF, WebP animado e GIF.
- **G2** — Abrir PPM/PNM (P1–P6; foco em P3 ASCII e P6 binário).
- **G3** — Associações de arquivo **opt-in**, por formato, desmarcadas por padrão, apenas avif marcado; nunca sequestrar
  outros formatos.
- **G4** — Prefetch bidirecional de bytes para navegação instantânea em pastas/sequências.
- **G5** — Funil discreto para o AVIFnim pago: modal único + banner contextual em GIF/WebP.
- **G6** — Ser pequeno, rápido, sem telemetria, MIT e multiplataforma.
  - **G7** — Ser útil para o público de render/VFX que exporta sequências (dar opção de visualizar sequencias de imagens png ou jpg animadas tocando em loop autoplay e na config escolhe se quer visualizar a 30 ou 60 fps as sequencias - só é necessario abrir um arquivo que ja detecta por nome a sequencia).

### 3.2 Não-objetivos (v1)

- Converter, editar, cortar, recortar ou exportar imagens (isso é o AVIFnim pago).
- Pausar, avançar frame a frame, buscar posição ou forçar loop infinito (v1 é loop-only).
- Reproduzir vídeo (MP4/MOV/etc.).
- Suportar HEIC, JXL, RAW ou PSD.
- Contas, pagamentos, assinatura, telemetria.
- Ser o visualizador padrão **à força**.
- Mobile (iOS/Android).
- HDR tone mapping avançado (avaliar em versão futura).

---

## 4. Personas

| Persona | Contexto | Necessidade |
|---|---|---|
| **Designer/dev** | Baixou um AVIF e o Windows não anima | Abrir com duplo clique e ver o loop |
| **Artista 3D/VFX** | Exporta sequência PPM do Blender/Nuke | Inspecionar frames e navegar rápido |
| **Usuário macOS** | Quick Look toca, mas com FPS limitado | Um viewer leve e previsível |
| **Usuário Linux** | Suporte a AVIF varia por distro | Um app simples que funcione |
| **Comprador potencial** | Vê GIF/WebP no viewer | Entender a economia do AVIF e conhecer o AVIFnim |

---

## 5. Proposta de valor e posicionamento

**Tagline:** “Visualise AVIF, WebP, GIF animados no seu PC”

- **Grátis e open source (MIT):** confiança, comunidade, sem telemetria.
- **Leve:** Tauri + webview do sistema.
- **Focado:** só visualização; nada de edição.
- **Útil para dois mundos:** web (AVIF/WebP/GIF) e render (PPM/PNM).
- **Respeitoso:** associações desmarcadas; só muda se o usuário pedir.
- **Funil honesto:** um modal único e um banner contextual; nada de paywall.

**Relação com o AVIFnim pago:** o viewer é o “test drive” do formato; o pago é o conversor
(sequências, vídeo, gravação de tela, GPU, CLI/MCP). O viewer **não** converte.

---

## 6. Escopo

### 6.1 v1.0 — MVP

- Windows 10 22H2+/11 (x64; ARM64 se o build for viável) vai ter versão instalavel dawnload pelo site INNO 6  não vai ser notarizadoe  instalador e versão pela micrsoft store.
- macOS 13 Ventura+ (Intel e Apple Silicon), não vai ser notarizado.
- Linux x64 (Ubuntu 24.04+/Fedora 40+, WebKitGTK 4.1) — **condicionado ao spike**; Flatpak é o
  canal preferencial.
- Formatos: AVIF (animado e still), WebP (animado e still), GIF, PPM/PNM (P1–P6).
- Loop conforme o arquivo; sem controles de reprodução.
- Navegação próximo/anterior com prefetch bidirecional.
- Zoom/pan, fit/100%, fullscreen, checkerboard, fundo claro/escuro.
- Associações opt-in por formato, desmarcadas.
- Modal promocional único + banner contextual em GIF/WebP.
- PT-BR e EN-US; tema claro/escuro/sistema.
- sem Auto-update  sem telemetria.
- MIT + avisos de terceiros.

### 6.2 v1.1 — evolução

- Extensão Quick Look no macOS (se superar o nativo).
- Flatpak oficial / listagem opcional na Microsoft Store.


### 6.3 Fora de escopo

- Pause/seek/frame step (só com decoder próprio; v2+).
- Conversão/edição/exportação.
- Vídeo, HEIC, JXL, RAW.
- Telemetria, contas, pagamentos.
- PPM no web viewer.
- Associação forçada ou “sequestro” de formatos.

---
O suporte ao PPM  vai ficar meio escondido pois as pessoas nem conhecem esse formado...
## 7. Requisitos funcionais

Prioridade: **P0** = MVP obrigatório; **P1** = importante; **P2** = desejável/versão futura.

### FR-01 — Abrir arquivos

| | |
|---|---|
| **Prioridade** | P0 |
| **Descrição** | Abrir por diálogo, drag & drop, “Abrir com”, argumento de linha de comando e evento do SO (macOS `open-file`). Instância única. |
| **Aceite** | Arquivo único, múltiplos arquivos e pasta abrem sem erro; segunda instância reutiliza a janela; nomes com espaços/unicode funcionam. |

### FR-02 — Formatos suportados

| | |
|---|---|
| **Prioridade** | P0 |
| **Descrição** | AVIF animado/still, WebP animado/still, GIF, PPM/PNM (P1–P6; P3/P6 prioritários). |
| **Aceite** | Cada formato abre e é identificado por badge; arquivo não suportado mostra mensagem amigável; PPM P3 e P6 exibem corretamente. |

### FR-03 — Reprodução em loop

| | |
|---|---|
| **Prioridade** | P0 |
| **Descrição** | Autoplay; loop conforme o loop count do arquivo; sem pause/seek/frame step. Stills ficam estáticos. |
| **Aceite** | AVIF/WebP/GIF animados tocam sozinhos; loop infinito do arquivo repete; loop 1x toca uma vez e para (comportamento documentado); sem controles de reprodução na UI. |

### FR-04 — Navegação

| | |
|---|---|
| **Prioridade** | P0 |
| **Descrição** | Próximo/anterior dentro da pasta, ordenação natural, setas do teclado, wrap-around opcional. |
| **Aceite** | Setas trocam de arquivo; ordem natural (frame_2 antes de frame_10); pasta com 300+ PPM navega sem travar. |

### FR-05 — Controles de visualização

| | |
|---|---|
| **Prioridade** | P1 |
| **Descrição** | Zoom in/out, fit, 100%, pan, fullscreen, checkerboard de alpha, fundo branco/preto/cinza, reset. |
| **Aceite** | Atalhos funcionam; zoom não distorce; checkerboard visível em AVIF/WebP/GIF com alpha; estado reseta ao trocar de arquivo (configurável). |

### FR-06 — Metadados

| | |
|---|---|
| **Prioridade** | P1 (básico) / P2 (avançado) |
| **Descrição** | Nome, formato, dimensões e tamanho (P1). Frames, duração, FPS e loop count (P2, requer parser de container). |
| **Aceite** | Painel “Info” mostra os campos disponíveis; não bloqueia a reprodução; valores corretos para AVIF/WebP/GIF/PPM. |

### FR-07 — Prefetch bidirecional

| | |
|---|---|
| **Prioridade** | P1 |
| **Descrição** | Cache dos arquivos vizinhos (atual ±N, default 2), em bytes comprimidos; prioridade para a direção do movimento; cancelamento; LRU; teto de memória (default 128 MB); início após o primeiro paint; ignora arquivos acima do limite. |
| **Aceite** | Troca para o próximo/anterior é instantânea em SSD; memória não ultrapassa o teto; navegação rápida não trava; cache limpa ao fechar. |

### FR-08 — Associações de arquivo

| | |
|---|---|
| **Prioridade** | P0 (política) |
| **Descrição** | Checkboxes por formato: AVIF, WebP, GIF, PPM/PNM. **Todas desmarcadas por padrão.** “Definir como padrão” abre as configurações do SO; o app nunca altera o padrão silenciosamente. Uninstall remove os registros. |
| **Aceite** | Instalação sem marcar nada não muda nenhum padrão; marcar AVIF registra só AVIF; desmarcar remove; outros formatos (PNG/JPG) nunca são tocados; uninstall limpa. |

### FR-09 — Funil para o AVIFnim

| | |
|---|---|
| **Prioridade** | P1 |
| **Descrição** | (a) Modal único após a 5°  visualização bem-sucedida, com “Conhecer o AVIFnim” ( SE ELE NAO ESTIVER INSTALADO NO SISTEMA) e “Não mostrar novamente”; (b) banner contextual ao abrir GIF/WebP com economia estimada e CTA. Sem bloquear, sem contagem regressiva, sem paywall. |
| **Aceite** | Modal aparece uma vez por instalação; dismiss persiste; link abre Store/site com UTM; banner é dispensável e não repete em loop; “Sobre” reabre o link. |  Mas no painel de about com as infos do Autor Everson Mayer vai ter banner destaque conheça AVIFnim com link que abre no browser e vai ter uma imagem animada em avifnim rodando.
Autor
Desenvolvido por Everson Mayer
E-mail
mayerdesigner@gmail.com
Site
** vou passar depois **
 colabore com o autor faça uma doação

 sobre o colabore coloque uma task sobre colocar o QR code do paypal e do PIX na versão em portugues igual o C2 - task pra eu te passar as imagens e infos
### FR-10 — Idioma e tema

| | |
|---|---|
| **Prioridade** | P1 |
| **Descrição** | PT-BR e EN-US; segue o sistema por padrão; tema claro/escuro/sistema. |
| **Aceite** | Troca de idioma sem reiniciar; textos sem cortes; tema acompanha o SO. |

### FR-11 — Teclado e acessibilidade

| | |
|---|---|
| **Prioridade** | P1 |
| **Descrição** | Setas (navegar), +/− (zoom), 0 (fit), F (fullscreen), Esc (sair), Ctrl/Cmd+O (abrir). Foco visível, labels ARIA, contraste AA. |
| **Aceite** | Todas as ações têm atalho e foco; leitor de tela anuncia nome/formato; sem armadilha de foco. |

### FR-12 — Atualizações

| | |
|---|---|
| **Prioridade** | P1 |
| **Descrição** | Checagem de update assinada; verificação manual no “Sobre”; sem telemetria. |
| **Aceite** | Update assinado é verificado; falha de rede não bloqueia; nenhum dado do usuário é enviado. |

### FR-13 — Erros

| | |
|---|---|
| **Prioridade** | P0 |
| **Descrição** | Arquivo corrompido, header inválido, dimensões absurdas, permissão negada e formato desconhecido mostram mensagem amigável e não derrubam o app. |
| **Aceite** | Fuzz básico não causa crash; cap de pixels evita OOM; erro permite tentar outro arquivo. |

### FR-14 — CLI (P2)

sem cli

### FR-15 — Quick Look (macOS, v1.1)

| | |
|---|---|
| **Prioridade** | P2 |
| **Descrição** | Extensão `QLPreviewingController` para tocar AVIF/WebP/GIF na barra de espaço, se entregar algo melhor que o Quick Look nativo (FPS/metadados). |
| **Aceite** | Instala/desinstala com o app; não vira padrão; funciona sandboxed; teste em Ventura/Sonoma. |

### FR-16 — Web viewer (v1.1)
não quero, nao faz sentido sendo que browser ja abre avif animado

### FR-17 — Configurações

| | |
|---|---|
| **Prioridade** | P1 |
| **Descrição** | Associações, tema, idioma, tamanho do prefetch, fundo padrão, “mostrar dicas”, reset do modal. |
| **Aceite** | Persistem entre sessões; reset restaura padrões; nada exige conta. |

---

## 8. Requisitos não funcionais

| Área | Requisito |
|---|---|
| **Desempenho** | Abrir AVIF de 5 MB < 300 ms (SSD); reprodução 1080p a 60 fps; startup < 1,0 s.se form muito longo testamos deixar uma base aberta em segundo plano no trayicon, meio que adormecido só pra nao demorar me mais a abrir imagens, mas isso deve ser teste para depois e task pra testar com abertura fria e abertura com app adormecido no trayicon |
| **Plataformas** | Windows 10  /11 x64 (ARM64 se viável); macOS 13+ Intel/Apple Silicon; Linux x64 com WebKitGTK 4.1 (Flatpak preferencial). deb focado em ubuntu e mint e talvez appimage se der pra difinir ou um script pra fazer o linux abri .avif com nosso viewr |
| **Segurança** | CSP restritiva; capabilities mínimas do Tauri; sem conteúdo remoto; sem shell; cap de pixels/bytes; fuzz de PPM/AVIF; updates assinados; decoder do sistema atualizado. |
| **Privacidade** | Sem telemetria; sem upload; única chamada de rede é a checagem de update (desativável). |
| **Acessibilidade** | Navegação por teclado, foco visível, contraste AA, respeito a `prefers-reduced-motion`. |
| **Licenciamento** | MIT; avisos de terceiros; **sem GPL** no binário (evitar FFmpeg; usar `image` crate para PNM). |
| **Confiabilidade** | Crash-free ≥ 99,5%; erros tratados; nenhum dado do usuário corrompido. |
| **Internacionalização** | PT-BR/EN-US; estrutura pronta para novos idiomas. |

---

## 9. Fluxos de UX

### 9.1 Primeira abertura

1. Janela com área de drop e botão “Abrir arquivo”.
2. Nenhum modal de onboarding; nenhuma associação automática.
3. Usuário arrasta um AVIF/WebP/GIF/PPM; o arquivo toca.
4. Após a primeira visualização, modal único do AVIFnim (dismissível).

### 9.2 Abrir pelo sistema

1. Duplo clique em `.avif` (se o usuário associou) ou “Abrir com”.
2. App abre na hora, com o arquivo; setas navegam a pasta.
3. Prefetch aquece os vizinhos.

### 9.3 GIF/WebP

1. Abre o GIF/WebP e toca.
2. Banner discreto: “Este GIF tem 4,2 MB. Em AVIF ficaria ~600 KB.” + “Converter com AVIFnim”.
3. “Agora não” dispensa; não repete na sessão.

### 9.4 Associações

1. Instalador (Windows/Linux) mostra checkboxes **desmarcados**.
2. Usuário marca o que quiser; “Definir como padrão” abre as configurações do SO.
3. No macOS, o app declara os tipos; o usuário escolhe no Finder; Ajustes tem o atalho.

### 9.5 Erro

1. Mensagem clara (“Não foi possível abrir este arquivo: cabeçalho PPM inválido”).
2. Botão “Abrir outro”; nenhum crash.

---

## 10. Arquitetura técnica

### 10.1 Stack

- **Tauri 2** (Rust) + UI HTML/CSS/JS; webview do sistema.
- **Rust** para: leitura de arquivos, cache de prefetch, decode PNM/PPM, configuração, updater.
- **`image` crate** (Rust puro) para PNM (P1–P6, 8/16 bits).
- **Sem FFmpeg, sem libavif, sem Electron** na v1.

### 10.2 Dois caminhos de renderização

| Tipo | Caminho | Motivo |
|---|---|---|
| AVIF/WebP/GIF animados | `<img>` com blob/asset URL | webview já decodifica e anima |
| PPM/PNM (still) | Rust decode → PNG → `<img>` | webview não suporta PPM |

- PPM é still; não entra no loop.
- 16-bit PNM é convertido para 8-bit na exibição.
- Cor: assumir sRGB (PPM não carrega perfil); documentar.

### 10.3 Prefetch

- Cache em Rust: `HashMap<Path, Arc<[u8]>>` + LRU + teto em MB.
- Janela ±N; prioridade para a direção; cancelamento por geração.
- Dispara depois do primeiro paint; não compete com o arquivo atual.
- Front recebe `blob:` URLs; revoga ao evictar.

### 10.4 Associações

- Tauri `bundle.fileAssociations` para os tipos suportados.
- Windows: NSIS checkboxes (opt-in) ou MSIX (usuário confirma).
- macOS: `CFBundleDocumentTypes` + `LSHandlerRank=Alternate`; UTIs do sistema ou UTI próprio conformando a `public.image`.
- Linux: `.desktop` com `MimeType=image/avif;image/webp;image/gif;image/x-portable-pixmap;image/x-portable-anymap;`; `xdg-mime` só sob ação.

### 10.5 Configuração

- JSON em app data: tema, idioma, tamanho do prefetch, associações, `promoSeen`, `bannerDismissed`.
- Sem nuvem; sem conta.

### 10.6 Empacotamento e distribuição

- Windows: NSIS (com checkboxes) e/ou MSIX; assinatura de código.
- macOS: DMG notarizado (Developer ID); mínimo Ventura.
- Linux: Flatpak (preferencial) e/ou deb/AppImage; WebKitGTK 4.1.
- Updates: Tauri updater com manifesto assinado; hospedar no site/releases.
- Ícones e metadados consistentes com a marca.

### 10.7 Web viewer

- HTML/CSS/JS estático, sem backend; drag & drop local; `<img>` para AVIF/WebP/GIF.
- Sem PPM na v1.
- SEO: páginas “abrir AVIF animado”, “visualizador WebP”, “abrir GIF”.
- CTA para o AVIFnim com UTM; sem links GitHub na landing.

### 10.8 Quick Look (v1.1)

- Appex + `QLPreviewingController`; WKWebView para teste rápido.
- Só evoluir para animador próprio se o Quick Look nativo não atender (FPS).
- Sandbox e notarização obrigatórias.

---

## 11. Riscos e mitigações

| Risco | Impacto | Mitigação |
|---|---|---|
| WebKitGTK sem AVIF animado | Linux não entrega | Spike; Flatpak com WebKitGTK conhecido; ou decoder próprio; ou lançar sem Linux |
| Loop count 1x para e não reinicia | Expectativa “sempre loop” | Documentar; testar; decoder v2 se necessário |
| Teto de FPS no WKWebView/Quick Look | Experiência pior | Medir; decoder/animador próprio só se justificar |
| PPM P3 ASCII gigante | Lentidão/memória | Cap de pixels/bytes; decode sob demanda; aviso |
| Header PPM malicioso | OOM/crash | Validar dimensões antes de alocar; fuzz |
| Cor P3/linear sem perfil | Cores erradas | Assumir sRGB; aviso; opção futura |
| Custo de assinatura/notarização | Fricção | Orçar cert Windows + Developer ID; Store como canal |
| Política da Store (link externo) | Rejeição | Linkar a listagem do AVIFnim na Store, não checkout |
| Scope creep (pause/seek/edição) | Atraso | Timebox; não-objetivos explícitos |
| Suporte de app grátis | Volume de issues | FAQ, MIT, sem SLA, issues no repo |
| Forks usarem a marca | Confusão | Política de marca; MIT não cobre trademark |
| Windows 10 EOL | Plataforma legada | Melhor esforço; foco Win11 |
| App grátis canibalizar o pago | Receita | Viewer só visualiza; CTA para converter |

---

## 12. Métricas de sucesso

- **Aquisição:** downloads do viewer (site + Store) 
 
 
- **Funil:** CTR do modal e do banner; conversões do AVIFnim com UTM.
- **Qualidade:** crash-free, tempo de abertura, memória, issues abertas.
 
- **Sem telemetria no app:** medir no site/Store; UTM nos links.
 
---

## 13. Cronograma e milestones

| Milestone | Entrega | Duração estimada |
|---|---|---|
| **M0 — Spike** | Matriz webview (Win/macOS/Linux), loop count, PPM decode, go/no-go Linux | 1–3 dias |
| **M1 — MVP Windows** | Skeleton, playback, prefetch, associações, modal, NSIS assinado | 2 semanas |
| **M2 — macOS** | DMG notarizado, UTIs, testes WKWebView | 1 semana |
| **M3 — Linux** | Flatpak/deb, WebKitGTK, testes (condicional) | 1 semana |
| **M4 — Web viewer + landing** | Página com drag & drop, SEO, downloads | 3–5 dias |
| **M5 — Polimento/OSS** | MIT, docs, QA, release, anúncio | 1 semana |
| **M6 — Quick Look** | Extensão macOS (se justificar) | 1–2 semanas |

Total: ~4–6 semanas em tempo parcial, sem contar imprevistos de Linux/assinatura.

---

## 14. Critérios de aceite do MVP
o instalador se possivel deve suportar escolher a pasta de instalação do software 
- [ ] Abre AVIF, WebP, GIF e PPM/PNM (P3/P6) sem erro.
- [ ] Toca animados em loop conforme o arquivo; stills estáticos.
- [ ] Setas navegam a pasta; prefetch bidirecional funciona; memória respeita o teto.
- [ ] Zoom/pan/fullscreen/checkerboard funcionam.
- [ ] Associações desmarcadas por padrão; opt-in por formato; uninstall limpa.
- [ ] Modal aparece uma vez; “não mostrar” persiste; banner de GIF/WebP é dispensável.
- [ ] PT-BR/EN-US; tema claro/escuro/sistema.
- [ ] Instaladores assinados/notarizados; update assinado.
- [ ] Sem telemetria; sem upload.
- [ ] MIT + avisos de terceiros.
- [ ] QA: arquivos malformados não derrubam; PPM 4K abre; unicode/espaços ok.
- [ ] Documentação de limitações (loop count, FPS, PPM sem perfil de cor).

---

## 15. Decisões já tomadas

- Viewer **gratuito** e **open source MIT**; 
- **Tauri 2 + Rust**, webview do sistema.
- **Loop-only** na v1; sem pause/seek/frame step.
- Formatos: **AVIF, WebP animado, GIF** + **PPM/PNM** still.
- Associações **opt-in**, **desmarcadas**, por formato; nunca outros formatos.
- **Prefetch bidirecional** de bytes (não de frames).
- **Modal único** + **banner contextual** em GIF/WebP.
- **Quick Look** como evolução (v1.1), não MVP.
- **Sem telemetria**.
- **Projeto separado** do AVIFnim; sem compartilhar código.
- **Web viewer** na landing (AVIF/WebP/GIF; PPM fora).

---
 
2. **Associações**: três/quatro checkboxes separados (recomendado) 
3. **Windows**: NSIS (opt-in fino) ou MSIX/Store (confiança/auto-update) — ou os dois? SIM 
4. **Linux**: Flatpak, AppImage ou deb? Linux entra na v1 sim na V1 e com deb.
5. **Metadados**: frames/duração/FPS entram na v1 ou ficam para v1.1? fica pra 1.1 
6. **Web viewer**: PPM fica fora da v1? (recomendação: sim) sim
7. **Repositório**: GitHub, A landing fornece o download e link do git (task final pra desenvolver essa pagina de download do software)  
8. **Modal**: aparece    após 5 views Texto e CTA é com vc e colca um link generico que depois te passo o link
 
10. **Marca/tagline**: “AVIFnim Viewer — para AVIF, WebP e GIFanimados”  
11. **macOS mínimo**: Ventura (13) é aceitável? sim 
12. **Windows 10**: suporte oficial ou “melhor esforço”? oficial - está sendo desenvolvido no windows 10 então facil de testar
13. **Open source**: política de marca/trademark e regras para forks. vc que sabe mais ai no github e nao no software né 
14. **Store**: listar o viewer grátis na Microsoft Store? sim 

---

## 17. Apêndices

### A. Matriz de suporte (2026)

| Visualizador | AVIF still | AVIF animado | WebP animado | GIF | PPM/PNM |
|---|---|---|---|---|---|
| Windows Fotos | sim (com AV1) | não | sim | sim | não |
| Windows Photo Viewer | não | não | não | sim | não |
| macOS Quick Look | sim | sim (FPS limitado) | sim | sim | não |
| macOS Preview | sim | primeiro frame | sim | sim | não |
| Linux Gwenview | sim | sim | sim | sim | parcial |
| Linux Loupe/eog | sim | inconsistente | sim | sim | não |
| Chrome/Edge/Firefox/Safari | sim | sim | sim | sim | não |
| **AVIFnim Viewer** | **sim** | **sim** | **sim** | **sim** | **sim** |

### B. Formatos

| Formato | Tipo | Animação | Alpha | Loop | Caminho no viewer |
|---|---|---|---|---|---|
| AVIF | still/animado | sim | sim | sim | `<img>` |
| WebP | still/animado | sim | sim | sim | `<img>` |
| GIF | animado | sim | 1-bit | sim | `<img>` |
| PPM (P3/P6) | still | não | não | não | Rust → PNG |
| PGM/PBM/PNM | still | não | não | não | Rust → PNG |

### C. Concorrentes

- **XnView MP** — grátis para uso pessoal; toca AVIF animado; muitos formatos; UI pesada.
- **Gwenview** — KDE; toca AVIF animado; Linux-only.
- **Navegadores** — universais; sem duplo clique/associação.
- **Quick Look** — integrado; FPS limitado; sem PPM.
- **ImageGlass/IrfanView/FastStone** — suporte variável a AVIF animado.

### D. Breakdown de tasks (não cadastradas)

| ID | Task | Depende | Tam. |
|---|---|---|---|
| `viewer-0` | Spike webview (AVIF/WebP/GIF/PPM, loop, FPS, Linux) | — | M |
| `viewer-1` | Esqueleto Tauri (argv, drop, instância única, i18n, tema) | 0 | S |
| `viewer-2` | Playback animado + still + PNM + UI | 0,1 | M |
| `viewer-2b` | Prefetch bidirecional (bytes, LRU, cap) | 2 | S/M |
| `viewer-3` | *(backlog)* decoder/player (pause/seek/loop forçado) | 2 | M/L |
| `viewer-4` | Associações por formato (opt-in, desmarcadas) | 2 | M |
| `viewer-5` | Modal + banner contextual | 2 | S |
| `viewer-6` | Pacotes, assinatura, updater | 2 | M |
| `viewer-7` | Web viewer + landing/SEO | 2 | M |
| `viewer-8` | QA/segurança (matriz, malformados, PPM, unicode) | 2,4,6 | M |
| `viewer-9` | Lançamento (MIT, marca, privacidade, notices, downloads) | 6,7,8 | S |
| `viewer-10` | Quick Look (macOS) | 3 | M/L |
| `viewer-11` | Open source setup (LICENSE, trademark, CI, releases) | 1 | S |
| `viewer-12` | Suporte PNM/PPM (decode/render/metadados) | 0 | S/M |
| `app-ppm` | *(AVIFnim pago)* aceitar PPM/PNM na sequência | — | S/M |

### E. Licenças de terceiros

- **Tauri** — MIT/Apache-2.0.
- **`image` crate** — MIT/Apache-2.0.
- **WebView2** — termos Microsoft (Windows).
- **WKWebView/WebKitGTK** — componentes do sistema.
- **Sem FFmpeg/GPL** na v1; se entrar no futuro, aplicar a mesma estratégia de processo separado +
  notices do AVIFnim.
- Avisos completos no “Sobre → Licenças” e no repositório.

### F. Glossário

- **AVIF** — AV1 Image File Format; still ou sequência animada.
- **WebP** — formato do Google; still ou animado.
- **GIF** — formato legado; animação com 256 cores.
- **PPM/PNM** — família Portable AnyMap; P3 = PPM ASCII, P6 = PPM binário.
- **UTI** — Uniform Type Identifier (macOS).
- **MIME** — tipo de mídia (Linux/Windows).
- **WebView2 / WKWebView / WebKitGTK** — webviews do Windows/macOS/Linux.
- **Prefetch** — leitura antecipada dos arquivos vizinhos para navegação instantânea.
- **LRU** — descarte do item menos usado recentemente.

---

*Fim do PRD. Documento independente do AVIFnim; copiar para o repositório do viewer antes de iniciar
o desenvolvimento.*
