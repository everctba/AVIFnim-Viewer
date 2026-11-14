# Changelog

Todas as mudanças notáveis deste projeto são documentadas aqui.
Formato baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/).

## [0.1.2] — 2026-11-13

### Alterado

- **Novo ícone do aplicativo** (design do owner, `icone.png` → ícones Windows/macOS/Linux).
- **Barra de ferramentas reorganizada e no rodapé**: abrir → nome do arquivo → ← → tela cheia → →
  → zoom − → **100%** → zoom + → **percentual de zoom** (o botão Fit foi removido; use 100% ou
  duplo clique para alternar Fit/100%).
- **Menu de contexto abre sempre para baixo** do clique (com rolagem interna se faltar espaço).
- **Modo tela cheia**: fundo 100% preto, barra de ferramentas oculta — apenas a imagem
  (Esc ou F para sair).
- **Menu nativo do webview suprimido**: sem imagem carregada não aparece nenhum menu; com imagem,
  o botão direito abre o menu do AVIFnim Viewer (campos de texto mantêm o menu nativo de copiar/colar).
- **Sem borda/anel branco de foco** ao redor do visualizador (o foco visível continua nos botões).
- **Modo branco (fundo claro) coeso**: toolbar/barra inferior e cartões passam a claros com ícones e
  textos escuros, e a tela vazia ganhou contraste (card sólido em vez de translúcido).
- **Tema com apenas 2 estados (Claro/Escuro)**: removido “Seguir o sistema” e o fundo independente
  (cinza); o tema controla a interface inteira, incluindo o fundo da área da imagem — no dark a barra
  fica escura. O botão de tema na barra inferior alterna Claro ↔ Escuro.

> Build de teste do Windows para validação visual; a versão final multiplataforma será
> reconstruída após o feedback (Linux/macOS ainda em 0.1.1 até lá).

## [0.1.1] — 2026-11-13

**Download:** [GitHub Releases — v0.1.1](https://github.com/everctba/AVIFnim-Viewer/releases/tag/v0.1.1)

### Adicionado

- **Menu de contexto próprio sobre a imagem** (substitui o menu nativo do webview):
  Abrir…, Abrir com… (Windows), Mostrar na pasta, Copiar caminho, Copiar imagem,
  Ajustar à janela, Tamanho real (100%), Mais/Menos zoom, Tela cheia, Informações,
  Configurações e Sobre. Fecha com Esc, clique fora ou clique em um item; navegável por teclado.
- **Duplo clique esquerdo alterna entre Fit e 100%**.
- Comandos cross-platform `reveal_in_folder` (Explorer/Finder/gerenciador) e
  `open_with_dialog` (Windows), com o item "Abrir com…" oculto nas outras plataformas.

### Alterado

- **Prefetch padrão agora é 3** arquivos para cada lado (antes 2).
- **Checkerboard de transparência desligado por padrão** (antes ligado); continua configurável.

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
