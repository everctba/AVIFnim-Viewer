# ADR 0002 — Notarização/assinatura do macOS

**Status:** Proposto (aguardando decisão do owner) — issue `avifnim-xqo`
**Data:** 2026-11-13

## Contexto

- PRD §6.1: “macOS 13 Ventura+ … **não vai ser notarizado**”.
- PRD §10.6: “macOS: DMG notarizado (Developer ID)”.
- PRD §14: “Instaladores assinados/notarizados”.
- Sem assinatura, o Gatekeeper bloqueia o app baixado da internet (“app danificado”/“desenvolvedor
  não identificado”), o que aumenta muito o atrito para um app gratuito de aquisição.

## Decisão recomendada

**Assinar com Developer ID e notarizar o DMG** (App Store Connect Notary Tool), mantendo
distribuição fora da Mac App Store. Custo: Apple Developer Program (US$ 99/ano) + pipeline
de notarização no CI.

Se o custo não for aceitável no MVP: publicar o DMG **sem assinatura** + instruções claras
(`xattr -d com.apple.quarantine`) e documentar como limitação; reavaliar na v1.1.

## Consequências

- Requer certificado, Team ID e segredos no GitHub Actions (`APPLE_CERTIFICATE`,
  `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID`).
- Atualizar PRD §6.1 para refletir a decisão.
- Mac App Store fica fora de escopo (o PRD prevê Quick Look como extensão na v1.1).

## Alternativas

- Distribuir apenas como código-fonte (`cargo build`) — inviável para o público-alvo.
- Mac App Store — políticas de sandbox conflitam com associações de arquivo e Quick Look futuro.
