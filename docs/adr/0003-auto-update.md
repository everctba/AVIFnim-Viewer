# ADR 0003 — Auto-update na v1

**Status:** Proposto (aguardando decisão do owner) — issue `avifnim-8hf`
**Data:** 2026-11-13

## Contexto

- PRD §6.1: “sem Auto-update, sem telemetria”.
- PRD §7 FR-12 (P1): “Checagem de update assinada; verificação manual no ‘Sobre’”.
- PRD §14: “Instaladores assinados/notarizados; update assinado”.

## Decisão recomendada

**v1 sem updater automático.** No “Sobre”, exibir a versão atual e um link para a página de
releases do GitHub (checagem manual). Isso mantém o app simples, sem chamadas de rede
automáticas e alinhado ao §6.1 (privacidade).

Se o owner quiser auto-update depois: usar `tauri-plugin-updater` com manifesto assinado
(`TAURI_SIGNING_PRIVATE_KEY`) hospedado nos releases — exige assinatura de código e
pipeline de release; mover para v1.1.

## Consequências

- Nenhuma requisição de rede no app na v1 (privacidade máxima).
- Usuário precisa baixar novas versões manualmente no site/releases.
- Documentar na política de privacidade e no README.
- Atualizar PRD para remover o FR-12 da v1 (ou movê-lo para v1.1).

## Alternativas

- Updater assinado completo: maior complexidade de CI e de segurança (chave de assinatura).
- Checagem de versão sem download automático: meio-termo, mas ainda é chamada de rede.
