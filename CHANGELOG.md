# Changelog

Todas as mudanças notáveis deste projeto são documentadas aqui.
Formato baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/).

## [0.1.0] — 2026-11-13

### Adicionado

- Esqueleto **Tauri 2 + Rust** com UI HTML/CSS/JS (sem bundler) e instância única.
- Abertura por diálogo nativo, drag & drop, argumento de linha de comando e evento `open-path`.
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
- **Funil AVIFnim**: modal único após a 5ª visualização + banner contextual em GIF/WebP.
- **Sobre** com autor, links, banner do AVIFnim e limitações conhecidas.
- **Robustez**: mensagens amigáveis para arquivos corrompidos/headers inválidos/dimensões absurdas.
- Testes unitários (14): decode PNM, cache LRU do prefetch e ordenação natural.
- Documentação: README, LICENSE (MIT), THIRD_PARTY_NOTICES, docs/BUILD e docs/QA.

### Notas

- Sem telemetria; a única chamada de rede futura será a checagem de update (decisão pendente).
- Assets oficiais (logo e links do site/AVIFnim) e QR codes de doação aguardam o owner.
