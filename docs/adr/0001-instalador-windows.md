# ADR 0001 — Instalador do Windows: Inno Setup 6 ou NSIS?

**Status:** Proposto (aguardando decisão do owner) — issue `avifnim-dd8`
**Data:** 2026-11-13

## Contexto

O PRD conflita: §6.1 cita “INNO 6” como instalador do Windows, enquanto §10.6 cita
“NSIS (com checkboxes)”. O aceite do MVP §14 pede escolha da pasta de instalação e
checkboxes de associação **desmarcados por padrão**. O projeto já usa o bundler do
Tauri 2, que gera **NSIS** nativamente.

## Decisão recomendada

**Manter NSIS** (já funcional: `AVIFnim Viewer_0.1.0_x64-setup.exe`, 1.31 MiB, instala em
`%LOCALAPPDATA%\AVIFnim Viewer`, desinstala limpo) e evoluir com **template NSIS
customizado** para:

1. página com escolha da pasta de instalação;
2. checkboxes opt-in de associação (desmarcados);
3. idiomas PT-BR/EN-US (já habilitados no `tauri.conf.json`).

Inno Setup 6 exigiria toolchain extra, um segundo pipeline e reescrever o empacotamento,
sem ganho funcional relevante para o MVP.

## Consequências

- `bundle.fileAssociations` foi **removido** do `tauri.conf.json` para o instalador não
  registrar associações silenciosamente — o opt-in é feito pelo app (Configurações),
  gravando em `HKCU\Software\Classes` (ProgID `AVIFnimViewer.*` + `OpenWithProgids`).
- Assinatura de código (certificado) permanece pendente de orçamento/decisão.
- Uninstall ainda não remove as associações criadas em runtime (limitação documentada;
  requer hook NSIS ou limpeza no desinstalador em iteração futura).

## Alternativas

- **Inno Setup 6**: UI de instalação mais rica, mas pipeline duplicado.
- **MSIX/Microsoft Store**: canal adicional v1.1 (`avifnim-269`), não substitui o instalador direto.
