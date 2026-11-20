# store-assets

Conteúdo para publicar o AVIFnim Viewer na Microsoft Store.

- `privacy.html` e `support.html`: copiar para o repositório `avifnim-site` e publicar.
- `screenshots/`: 5 imagens 1920×1080 prontas para o Partner Center.
- `listing.md`: descrições, keywords, copyright, requisitos, classificação e notas de certificação.
- `assets/app-icon-512.png`: ícone de apoio das páginas (o site já possui esse arquivo).

O build do app (NSIS, portátil e MSIX) fica em `dist/`; o MSIX é gerado por
`scripts/build-msix.ps1` e pelo workflow `MSIX (Microsoft Store)`.
