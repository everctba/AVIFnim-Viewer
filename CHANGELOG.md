# Changelog

Todas as mudanças notáveis deste projeto são documentadas aqui.
Formato baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/).

## [0.1.0] — 2026-11-13

**Download:** [GitHub Releases — v0.1.0](https://github.com/everctba/AVIFnim-Viewer/releases/tag/v0.1.0)
(instalador NSIS para Windows 10/11 x64; sem assinatura de código nesta versão).

### Adicionado

- Esqueleto **Tauri 2 + Rust** com UI HTML/CSS/JS (sem bundler) e instância única.
- Abertura por diálogo nativo, drag & drop, argumento de linha de comando e evento `open-path`.
- Título nativo da janela acompanhando o arquivo aberto.
- Modo de benchmark (`AVIFNIM_BENCH=1`) com log JSON (`first_paint`, `sequence_fps`).
- **Reprodução** de AVIF, WebP e GIF animados via webview do sistema (autoplay + loop do arquivo).
- **Decode PPM/PNM (P1–P6, 8/16-bit)** em Rust puro (`image`), com caps de pixels/bytes e conversão para PNG.
- **Navegação** na pasta com ordenação natural, setas do teclado e wrap-around.
- **Prefetch bidirecional** de bytes com cache LRU e teto configurável (default 128 MB).
- **Controles de visualização**: zoom, pan, fit, 100%, fullscreen, checkerboard e fundos.
- **Painel de informações** (nome, formato, dimensões, tamanho, sequência, caminho).
- **Sequências PNG/JPG (G7)**: detecção automática por padrão de nome, loop a 30/60 fps.
- **Configurações persistentes** (tema, idioma, prefetch, fundo, checkerboard, dicas, FPS).
- **PT-BR/EN-US** e tema claro/escuro/sistema (inclusive title bar nativa).
- **Associações de arquivo opt-in** por formato (desmarcadas), via registro do Windows (HKCU), sem sequestrar padrões.
- **Instalador NSIS customizado**: página de associações opt-in (4 checkboxes desmarcados), escolha da pasta e suporte a `/ASSOC` para instalações silenciosas.
- **Uninstall limpa as associações** criadas pelo app (hook NSIS `NSIS_HOOK_PREUNINSTALL`).
- Opção de **resetar zoom/pan ao trocar de arquivo** (configurável, FR-05).
- **Build Linux `.deb`** validado no CI (incluindo smoke test do app sob Xvfb) e publicado no release.
- **Funil AVIFnim**: modal único após a 5ª visualização + banner contextual em GIF/WebP.
- **Sobre** com autor, links, banner do AVIFnim e limitações conhecidas.
- **Robustez**: mensagens amigáveis para arquivos corrompidos/headers inválidos/dimensões absurdas.
- Testes unitários (14): decode PNM, cache LRU do prefetch e ordenação natural.
- Documentação: README, LICENSE (MIT), THIRD_PARTY_NOTICES, docs/BUILD e docs/QA.

### Notas

- Sem telemetria; a única chamada de rede futura será a checagem de update (decisão pendente).
- Assets oficiais (logo e links do site/AVIFnim) e QR codes de doação aguardam o owner.
