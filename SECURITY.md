# Security Policy — AVIFnim Viewer

## Versões suportadas

| Versão | Suporte de segurança |
|---|---|
| 0.1.x | ✅ (versão atual) |

## Como reportar uma vulnerabilidade

**Não abra issue pública.** Prefira o canal privado do GitHub: **Security → Report a
vulnerability** (se o private vulnerability reporting estiver habilitado no repositório).
Alternativa: envie um e-mail para **mayerdesigner@gmail.com** com:

- versão do app, plataforma (Windows/Linux/macOS) e forma de instalação;
- passos para reproduzir;
- impacto estimado (ex.: leitura de arquivos, execução de código, negação de serviço);
- prova de conceito, se houver (sem dados sensíveis).

Resposta em até ~7 dias. Se confirmado, corrigimos e publicamos uma versão com crédito
(opcional) no CHANGELOG/release.

## Escopo

O AVIFnim Viewer é um **visualizador local**: não tem conta, telemetria nem rede (a única ação de
rede é abrir um link no navegador). A superfície de ataque principal é:

1. **Decodificação de imagens** pelo webview do sistema (WebView2 / WKWebView / WebKitGTK) e pelo
   crate `image` (PNM/PPM) no backend Rust.
2. **Renderer comprometido** → comandos IPC expostos.
3. **Cadeia de suprimentos** (dependências, CI, binários distribuídos).

## Medidas implementadas

- **Sem rede no app**, CSP restritiva, sem conteúdo remoto, sem `eval`.
- **`open_url` com allowlist** (GitHub do projeto, site oficial `avifnim.com.br`, página de doação
  do PayPal e o e-mail do autor); URLs arbitrárias são rejeitadas — evita exfiltração caso o
  renderer seja comprometido.
- **`reveal_in_folder` / `open_with_dialog`** validam que o caminho é um **arquivo de imagem
  suportado** (sem abrir qualquer arquivo do disco).
- **Caps de segurança**: arquivo ≤ 256 MB, PNM ≤ 100 MPix, validação de header antes de decodificar.
- **Sem shell**; processos externos são chamados com argumentos (sem concatenação de comando).
- **Associações opt-in** e desmarcadas; o app nunca altera padrões silenciosamente.
- **CI de supply chain**: `cargo audit`, `cargo deny` (advisories + licenças, exceções documentadas),
  `npm audit`, **SBOM CycloneDX** como artefato, Actions **pinadas por commit SHA** e Dependabot.
- **SAST (CodeQL)**: análise estática de JavaScript/TypeScript em push/PR e semanal, com resultados
  na aba **Security → Code scanning**; Rust segue coberto por `clippy -D warnings`, `cargo audit` e
  `cargo deny` (o extrator Rust do CodeQL ainda é preview).
- **Secret scanning (gitleaks)**: varredura de segredos em todo push/PR e no histórico completo do
  git (`fetch-depth: 0`), com falha de CI se encontrar algo; varredura local: 46 commits, 0 vazamentos.
- **Fuzzing/property tests**: 4 proptests no parser PNM (bytes arbitrários, cap de 100 MPix,
  whitespace/comentários, headers truncados) e alvo `cargo-fuzz` rodando 60 s no CI.
- Testes: **30 testes unitários** (incluindo allowlist de URLs e validação de caminhos).

## Limitações conhecidas / pendências

- Instaladores **sem assinatura de código** (Windows Authenticode) e **sem notarização** (macOS) —
  decisões/certificados pendentes com o owner.
- Dependências transitivas do stack GTK/zbus marcadas como *unmaintained* pelo RustSec
  (`RUSTSEC-2024-0370`, `RUSTSEC-2026-0370/0371/0372`) sem upgrade seguro disponível no Tauri atual —
  registradas em `src-tauri/deny.toml`.
- Distribuição Linux ainda em `.deb` sem sandbox; Flatpak sandboxed + repo assinado estão no roadmap.
- **CodeQL para Rust** não habilitado (extrator em preview e build exige stack GTK/WebKit).
- **Branch protection** e **secret scanning/push protection** nativos do GitHub ainda precisam ser
  ativados nas configurações do repositório (owner).
- Não há programa de bug bounty.
