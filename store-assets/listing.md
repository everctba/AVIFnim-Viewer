# AVIFnim Viewer — listagem Microsoft Store

Pasta de entrega para o Partner Center. Nada aqui é usado pelo build do app; é conteúdo de loja.

## Arquivos

| Arquivo | Uso |
|---|---|
| `privacy.html` | Política de privacidade (publicar em `avifnim-site/privacy.html`) |
| `support.html` | Página de suporte (publicar em `avifnim-site/support.html`) |
| `assets/app-icon-512.png` | Ícone usado pelas páginas (o site já tem `assets/app-icon-512.png`) |
| `screenshots/01-visualizador.png` … `05-configuracoes.png` | 5 screenshots 1920×1080 para a Store |
| `listing.md` | Textos e respostas para preencher o Partner Center |

> As páginas `privacy.html` e `support.html` precisam ser copiadas para o repositório
> `avifnim-site` e publicadas; até lá as URLs não existem.

## URLs para o Partner Center

- **Privacy policy URL:** `https://everctba.github.io/avifnim-site/privacy.html`
- **Support URL:** `https://everctba.github.io/avifnim-site/support.html`
- **Support email:** mayerdesigner@gmail.com
- **GitHub Issues:** https://github.com/everctba/AVIFnim-Viewer/issues
- **License terms (opcional):** https://github.com/everctba/AVIFnim-Viewer/blob/master/LICENSE

## Screenshots — legendas sugeridas

| Arquivo | Legenda PT-BR | Caption EN-US |
|---|---|---|
| `01-visualizador.png` | Visualizador de imagens animadas em tela cheia, com navegação e zoom | Animated image viewer with navigation and zoom |
| `02-sequencias-autoplay.png` | Sequências de PNG/JPG em loop, com Auto play ligado | PNG/JPG sequences looping with Auto play on |
| `03-tema-claro.png` | Tema claro e interface limpa | Light theme and clean interface |
| `04-zoom.png` | Zoom, pan e visualização em detalhe | Zoom, pan and detail view |
| `05-configuracoes.png` | Configurações de aparência, idioma e desempenho | Appearance, language and performance settings |

Todas em **1920×1080 PNG**, sem caminhos pessoais e sem molduras de dispositivo.

## Descrição curta (PT-BR)

Visualizador gratuito e open source de AVIF, WebP e GIF animados, PPM/PNM e sequências de imagens.
Leve, sem telemetria e sem cadastro.

## Descrição completa (PT-BR)

O AVIFnim Viewer é um visualizador gratuito e open source (MIT) para imagens animadas e sequências.
Abra AVIF, WebP e GIF animados, PPM/PNM e sequências numeradas de PNG/JPG — com loop, navegação de
pasta e controles de zoom.

**Recursos principais**
- AVIF animado e still, WebP, GIF e PPM/PNM (P1–P6)
- Sequências PNG/JPG detectadas pelo nome, com loop a 30 ou 60 fps e pílula Auto play
- Navegação por setas, ordenação natural (`frame_2` antes de `frame_10`) e prefetch dos vizinhos
- Zoom, pan, Fit, 100%, tela cheia (F11), checkerboard de transparência
- Tema claro/escuro e interface em português (BR) ou inglês
- Sem telemetria, sem contas, sem nuvem e sem anúncios

O AVIFnim Viewer não envia seus arquivos para servidores. Tudo é processado localmente no seu PC.

## Short description (EN-US)

Free and open-source viewer for animated AVIF, WebP and GIF, PPM/PNM and numbered image sequences.
Lightweight, no telemetry, no sign-up.

## Full description (EN-US)

AVIFnim Viewer is a free, open-source (MIT) viewer for animated images and image sequences. Open
animated AVIF, WebP and GIF, PPM/PNM, and numbered PNG/JPG sequences with looping, folder navigation
and zoom controls.

**Key features**
- Animated and still AVIF, WebP, GIF and PPM/PNM (P1–P6)
- PNG/JPG sequences detected by filename, looping at 30 or 60 fps with an Auto play toggle
- Arrow-key navigation, natural sorting and neighbor prefetch
- Zoom, pan, Fit, 100%, fullscreen (F11), transparency checkerboard
- Light/dark theme and Portuguese (BR) or English UI
- No telemetry, no accounts, no cloud and no ads

AVIFnim Viewer does not upload your files. Everything is processed locally on your PC.

## Categoria

**Photo & video** (PT-BR: **Fotos e vídeos**).
Alternativa aceitável: Utilities & tools.

## Keywords

**PT-BR (até 7):**
`visualizador avif`, `avif animado`, `visualizador webp`, `gif animado`, `visualizador de imagens`,
`sequências de imagens`, `open source`

**EN-US (até 7):**
`avif viewer`, `animated avif`, `webp viewer`, `gif viewer`, `image viewer`, `image sequences`,
`open source`

## Copyright

- **PT-BR:** © 2026 Everson Mayer. Todos os direitos reservados.
- **EN-US:** © 2026 Everson Mayer. All rights reserved.

## Classificação etária

Questionário IARC: sem violência, sexo, drogas, apostas, linguagem imprópria, compras, anúncios ou
interação online; sem coleta de dados pessoais. Resultado esperado: **Everyone** (IARC) /
**Livre** (ClassInd).

## Requisitos de sistema

- Windows 10 versão 1809 (build 17763) ou superior; Windows 11
- x64
- Processador x64 1,6 GHz+
- 2 GB de RAM (4 GB recomendado)
- 100 MB de disco
- Gráficos integrados
- **WebView2 Runtime** (Windows 11 já inclui; Windows 10 normalmente via Edge/Windows Update)
- Internet não é necessária para usar; apenas para abrir links externos clicados

## Notas para certificação (Partner Center)

> Cole no campo “Notes for certification” (limite ~2.000 caracteres).

AVIFnim Viewer is a free, open-source desktop image viewer. It opens local image files (animated and
still AVIF, WebP, GIF, PPM/PNM, PNG/JPG) and numbered PNG/JPG sequences. Core functionality does not
require network access. There are no accounts, ads, in-app purchases, telemetry or user-generated
content. The app uses the Microsoft Edge WebView2 Runtime. It runs as a full-trust desktop app
because it needs local file/folder access to open images, navigate sibling files and prefetch
neighbors. File associations are optional; in the MSIX package the association UI is hidden because
package identity manages them. Privacy policy: https://everctba.github.io/avifnim-site/privacy.html
Support: https://everctba.github.io/avifnim-site/support.html
Source code (MIT): https://github.com/everctba/AVIFnim-Viewer
Test files: open any local AVIF/WebP/GIF/PNM/PNG/JPG; the app also detects `frame_01.png`,
`frame_02.png`… sequences in the same folder.

## O que há de novo (primeira versão)

- **PT-BR:** Versão inicial para Microsoft Store. Visualizador gratuito de AVIF, WebP, GIF animados
  e sequências de imagens.
- **EN-US:** Initial Microsoft Store release. Free viewer for animated AVIF, WebP, GIF and image
  sequences.
