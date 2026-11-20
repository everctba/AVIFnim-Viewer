# Microsoft Store — respostas para o Partner Center

Página provisória do site: https://everctba.github.io/avifnim-site/viewer.html

## 1. Política de privacidade (URL)

**URL sugerida:** `https://everctba.github.io/avifnim-site/privacy.html`

> ⚠️ Essa página ainda precisa existir no site. Se ela não estiver publicada, o Partner Center
> pode rejeitar o link. O texto completo está no fim deste documento; publique como `privacy.html`.

Alternativa temporária (até publicar): `https://github.com/everctba/AVIFnim-Viewer/blob/master/PRIVACY.md`
— funciona, mas uma página do site é mais adequada para a Store.

## 2. Site de suporte

**URL:** `https://everctba.github.io/avifnim-site/viewer.html`

Complementos recomendados:

- **Issues:** https://github.com/everctba/AVIFnim-Viewer/issues
- **E-mail:** mayerdesigner@gmail.com

> Ideal: criar `https://everctba.github.io/avifnim-site/support.html` com o texto de suporte abaixo
> e usar essa URL. A página `viewer.html` serve como provisória.

## 3. Categoria

**Photo & video** (em PT-BR: **Fotos e vídeos**)

Alternativa aceitável: **Utilities & tools** (Utilitários e ferramentas). Para um visualizador de
imagens, “Fotos e vídeos” é a mais adequada.

## 4. Descrição

### PT-BR — descrição curta

Visualizador gratuito e open source de AVIF, WebP e GIF animados, PPM/PNM e sequências de imagens.
Leve, sem telemetria e sem cadastro.

### PT-BR — descrição completa

O AVIFnim Viewer é um visualizador gratuito e open source (MIT) para imagens animadas e sequências.
Abra AVIF, WebP e GIF animados, PPM/PNM e sequências numeradas de PNG/JPG — com loop, navegação de
pasta e controles de zoom.

**Recursos principais**
- AVIF animado e still, WebP, GIF e PPM/PNM (P1–P6)
- Sequências PNG/JPG detectadas pelo nome, com reprodução em loop a 30 ou 60 fps e pílula Auto play
- Navegação por setas, ordenação natural (`frame_2` antes de `frame_10`) e prefetch dos vizinhos
- Zoom, pan, Fit, 100%, tela cheia (F11), checkerboard de transparência
- Tema claro/escuro e interface em português (BR) ou inglês
- Sem telemetria, sem contas, sem nuvem e sem anúncios

O AVIFnim Viewer não envia seus arquivos para servidores. Tudo é processado localmente no seu PC.
Ideal para conferir AVIF animado, GIFs, WebP e sequências de quadros sem abrir um editor pesado.

### EN-US — short description

Free and open-source viewer for animated AVIF, WebP and GIF, PPM/PNM and numbered image sequences.
Lightweight, no telemetry, no sign-up.

### EN-US — full description

AVIFnim Viewer is a free, open-source (MIT) viewer for animated images and image sequences. Open
animated AVIF, WebP and GIF, PPM/PNM, and numbered PNG/JPG sequences with looping, folder
navigation and zoom controls.

**Key features**
- Animated and still AVIF, WebP, GIF and PPM/PNM (P1–P6)
- PNG/JPG sequences detected by filename, looping at 30 or 60 fps with an Auto play toggle
- Arrow-key navigation, natural sorting (`frame_2` before `frame_10`) and neighbor prefetch
- Zoom, pan, Fit, 100%, fullscreen (F11), transparency checkerboard
- Light/dark theme and Portuguese (BR) or English UI
- No telemetry, no accounts, no cloud and no ads

AVIFnim Viewer does not upload your files. Everything is processed locally on your PC. Great for
checking animated AVIF, GIFs, WebP and frame sequences without opening a heavy editor.

## 5. Screenshots

Formato recomendado: **PNG, 1920×1080**, mínimo 1366×768, até 10 imagens. Sem molduras de celular
(é app desktop). Legendas sugeridas:

1. **AVIF animado em loop** — imagem colorida com a barra de sequência e o badge AVIF.
2. **Sequências da pasta** — vários `frame_01..04` com a pílula **Auto play: ligado**.
3. **Tema claro** — mesma tela no tema claro, mostrando o contraste da interface.
4. **Tela cheia e zoom** — imagem ampliada com o checkerboard de transparência.
5. **Configurações** — seção de aparência/idioma/desempenho.

Dica: não mostrar caminhos pessoais (`C:\Users\...`) nem nomes privados nas capturas.

## 6. Classificação etária

Preencha o questionário **IARC** do Partner Center respondendo:

- Sem violência, sexo, drogas, apostas, linguagem imprópria ou conteúdo adulto
- Sem compras, anúncios ou interação online
- Sem coleta de dados pessoais
- Aplicativo utilitário de visualização de imagens locais

Resultado esperado: **Everyone** (IARC) / **Livre** (ClassInd). A classificação final é a que o
questionário emitir; o app em si não distribui conteúdo.

## 7. Requisitos de sistema

Preencha a seção de requisitos da Store com:

- **Sistema operacional:** Windows 10 versão 1809 (build 17763) ou superior; Windows 11
- **Arquitetura:** x64
- **Processador:** x64 1,6 GHz ou superior
- **Memória:** 2 GB (4 GB recomendado)
- **Disco:** 100 MB
- **Gráficos:** integrado (sem GPU dedicada)
- **Dependência:** **Microsoft Edge WebView2 Runtime** (incluído no Windows 11; no Windows 10,
  normalmente presente via Edge/Windows Update)
- **Conexão:** não é necessária para usar o app; apenas para abrir links externos (site, GitHub,
  PayPal) quando o usuário clica

---

# Texto da Política de Privacidade (para publicar em `privacy.html`)

**Política de Privacidade — AVIFnim Viewer**
Última atualização: 20 de novembro de 2026

O AVIFnim Viewer é um visualizador de imagens local. Esta política explica quais dados o
aplicativo trata.

**1. Dados que coletamos**
O AVIFnim Viewer não coleta, armazena, transmite nem vende dados pessoais. Não há contas,
telemetria, analytics, anúncios ou rastreamento.

**2. Arquivos e imagens**
As imagens e pastas abertas são processadas localmente no seu dispositivo. Nenhum arquivo é
enviado para servidores nossos ou de terceiros.

**3. Configurações**
Preferências (tema, idioma, zoom, FPS de sequência etc.) são salvas localmente na pasta de dados do
aplicativo no Windows (`%APPDATA%\com.avifnim.viewer`). Elas não são sincronizadas nem enviadas
para a nuvem.

**4. Rede e links externos**
O aplicativo não acessa a internet para funcionar. Quando você clica em links para o site, GitHub
ou PayPal, o link é aberto no seu navegador padrão; a partir daí valem as políticas desses sites.

**5. Microsoft Store**
Se você instalar o aplicativo pela Microsoft Store, a Microsoft poderá coletar dados de uso e
diagnóstico conforme a política de privacidade da própria Microsoft Store. Não temos acesso a
dados pessoais coletados pela Store.

**6. Crianças**
O aplicativo não coleta dados pessoais e é adequado para todas as idades.

**7. Alterações**
Se esta política mudar, a versão atualizada será publicada nesta página com nova data.

**8. Contato**
Dúvidas sobre privacidade: mayerdesigner@gmail.com

---

# Texto do site de suporte (para publicar em `support.html`)

**Suporte — AVIFnim Viewer**

Precisa de ajuda com o AVIFnim Viewer?

- **Perguntas frequentes e download:** https://everctba.github.io/avifnim-site/viewer.html
- **Problemas e sugestões:** https://github.com/everctba/AVIFnim-Viewer/issues
- **E-mail:** mayerdesigner@gmail.com

Ao reportar um problema, informe: versão do app, versão do Windows e o formato do arquivo
(AVIF, WebP, GIF, PPM/PNM, PNG/JPG). Se possível, anexe uma captura de tela.
