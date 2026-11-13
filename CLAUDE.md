# Project Instructions for AI Agents

This file provides instructions and context for AI coding agents working on this project.

<!-- BEGIN BEADS INTEGRATION v:1 profile:minimal hash:7510c1e2 -->
## Beads Issue Tracker

This project uses **bd (beads)** for issue tracking. Run `bd prime` to see full workflow context and commands.

### Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --claim  # Claim work
bd close <id>         # Complete work
```

### Rules

- Use `bd` for ALL task tracking — do NOT use TodoWrite, TaskCreate, or markdown TODO lists
- Run `bd prime` for detailed command reference and session close protocol
- Use `bd remember` for persistent knowledge — do NOT use MEMORY.md files

**Architecture in one line:** issues live in a local Dolt DB; sync uses `refs/dolt/data` on your git remote; `.beads/issues.jsonl` is a passive export. See https://github.com/gastownhall/beads/blob/main/docs/SYNC_CONCEPTS.md for details and anti-patterns.

## Session Completion

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds
<!-- END BEADS INTEGRATION -->


## Build & Test

```bash
npm install
scripts\build.bat test    # cargo test (Windows/MSVC) — 24 testes
scripts\build.bat build   # build release
npx tauri build           # instalador NSIS em src-tauri/target/release/bundle/nsis
scripts\build.ps1 test    # wrapper PowerShell (localiza o vcvars64 via vswhere)
```

Linux/macOS: veja `docs/BUILD.md`.

## Architecture Overview

UI em `src/` (HTML/CSS/JS puro, sem bundler) ↔ comandos Tauri em `src-tauri/src/lib.rs`.
Rust faz leitura de arquivos, decode PPM/PNM (`pnm.rs`), cache LRU do prefetch (`cache.rs`),
configurações (`config.rs`) e associações do Windows (`assoc.rs`). AVIF/WebP/GIF são
decodificados/animados pelo webview do sistema via `<img>` com data URL.

## Conventions & Patterns

- Sem FFmpeg/GPL no binário; PNM via crate `image`.
- Configurações em JSON no app data; sem telemetria e sem rede na v1.
- Associações opt-in (HKCU) e nunca alterar o padrão do sistema silenciosamente.
- Textos em PT-BR/EN-US (i18n em `src/main.js`, chaves `data-i18n`).
- Fechar issues no bd e rodar `git push` no fim da sessão.
