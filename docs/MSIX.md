# MSIX para Microsoft Store

Este diretório contém o **empacotamento MSIX do AVIFnim Viewer**. Ele é **independente** do
instalador NSIS (`.exe`) e da versão portátil — nada no build atual é alterado.

## Identidade do pacote (Partner Center)

| Campo | Valor |
|---|---|
| Package/Identity/Name | `EversonMayer.AVIFnimViewer` |
| Package/Identity/Publisher | `CN=1D858FBC-162E-499E-A62D-E5B3D2AA6774` |
| PublisherDisplayName | `Everson Mayer` |
| Versão | `0.1.2.0` (derivada de `0.1.2`) |

PFN, SID, Store ID e link profundo **não** entram no `AppxManifest.xml`; são gerados/gerenciados
pelo Partner Center.

## Gerar localmente

Requisitos:

- Windows 10/11 x64
- Node.js + dependências (`npm ci`)
- Rust MSVC
- **Windows SDK** com `makeappx.exe` e `signtool.exe`

```powershell
powershell -ExecutionPolicy Bypass -File scripts/build-msix.ps1
```

Saída:

```text
dist\AVIFnim.Viewer_0.1.2_x64.msix
```

### Assinar (sideload/teste)

```powershell
# com PFX
powershell -ExecutionPolicy Bypass -File scripts/build-msix.ps1 `
  -CertificatePath "C:\certs\avifnim.pfx" `
  -CertificatePassword "senha"

# com certificado no repositório do Windows
powershell -ExecutionPolicy Bypass -File scripts/build-msix.ps1 `
  -CertificateThumbprint "ABCDEF..."
```

Sem certificado o script gera o MSIX **sem assinatura**:

- para upload na Store, o Partner Center valida a identidade; a Store assina o pacote publicado;
- para instalar localmente fora da Store, é preciso assinar com um certificado confiável (o
  certificado precisa ter o mesmo `CN` do Publisher do manifesto).

## WebView2

O app usa o **WebView2 Runtime do sistema**:

- Windows 11: já vem instalado;
- Windows 10: normalmente presente via Edge/atualizações; se não estiver, o WebView2 pode ser
  instalado pelo instalador do Windows Update/Edge.

Se a Store exigir ou se quisermos suportar Windows 10 sem WebView2, o próximo passo é incluir o
**Evergreen Bootstrapper** ou um runtime fixo no pacote (aumenta bastante o tamanho).

## Publicar no Partner Center

1. Gere o `.msix` (idealmente assinado).
2. Partner Center → seu produto → **Pacotes** → envie o `.msix`.
3. Preencha/valide:
   - Propriedades (categoria, requisitos de sistema: Windows 10/11 x64)
   - Classificação etária
   - Listagem da Store (descrição, screenshots, ícone, preço grátis)
   - Política de privacidade (URL) e suporte
4. Envie para certificação.
5. Após aprovação, o link profundo e a URL da Web Store aparecem no Partner Center.

## Diferenças de comportamento no MSIX

- **Associações de arquivo**: no MSIX o app roda com identidade de pacote; a seção de associações
  do Windows é **ocultada** quando `GetCurrentPackageFullName` indica pacote (o app detecta e
  ajusta a UI). Para associar no MSIX, o caminho correto seria declarar extensões no manifesto
  (não habilitado nesta versão para não conflitar com o instalador NSIS).
- **Instalação lado a lado**: não instale NSIS e MSIX na mesma máquina ao mesmo tempo; podem
  aparecer duas entradas/atalhos.
- **Versão**: o MSIX usa 4 partes (`0.1.2.0`); o NSIS continua com `0.1.2`.

## Workflow

`.github/workflows/msix.yml` (workflow_dispatch) gera o `.msix` no Windows runner e sobe como
artifact `windows-msix`. O workflow do instalador/CI atual não foi alterado.
