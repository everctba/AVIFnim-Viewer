param(
  [string]$Version = "",
  [switch]$SkipBuild,
  [string]$CertificatePath = "",
  [string]$CertificatePassword = "",
  [string]$CertificateThumbprint = "",
  [string]$TimestampUrl = "http://timestamp.digicert.com"
)

$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

function Find-SdkTool([string]$Name) {
  $cmd = Get-Command $Name -ErrorAction SilentlyContinue
  if ($cmd) { return $cmd.Source }
  $roots = @(
    "${env:ProgramFiles(x86)}\Windows Kits\10\bin",
    "$env:ProgramFiles\Windows Kits\10\bin"
  )
  foreach ($r in $roots) {
    if (-not (Test-Path $r)) { continue }
    $found = Get-ChildItem $r -Recurse -Filter $Name -ErrorAction SilentlyContinue |
      Where-Object { $_.FullName -match '\\x64\\' } |
      Sort-Object FullName -Descending |
      Select-Object -First 1
    if ($found) { return $found.FullName }
  }
  return $null
}

if (-not $Version) {
  $conf = Get-Content "src-tauri/tauri.conf.json" -Raw | ConvertFrom-Json
  $Version = "$($conf.version).0"
}
if ($Version -notmatch '^\d+\.\d+\.\d+\.\d+$') {
  throw "Versao MSIX invalida: '$Version' (use 4 partes, ex.: 0.1.2.0)"
}
$displayVersion = ($Version.Split(".")[0..2] -join ".")

if (-not $SkipBuild) {
  Write-Host "==> Build release (no bundle)"
  & npx tauri build --no-bundle
  if ($LASTEXITCODE -ne 0) { throw "tauri build falhou ($LASTEXITCODE)" }
}

$exe = Join-Path $root "src-tauri/target/release/avifnim-viewer.exe"
if (-not (Test-Path $exe)) { throw "Executavel nao encontrado: $exe" }

$stage = Join-Path $root "src-tauri/target/msix/layout"
if (Test-Path $stage) { Remove-Item $stage -Recurse -Force }
New-Item -ItemType Directory -Path $stage | Out-Null
New-Item -ItemType Directory -Path (Join-Path $stage "Assets") | Out-Null

Copy-Item $exe (Join-Path $stage "avifnim-viewer.exe") -Force
Copy-Item "msix/assets/*.png" (Join-Path $stage "Assets") -Force

$manifest = Get-Content "msix/AppxManifest.xml" -Raw
$manifest = $manifest.Replace("{{VERSION}}", $Version).Replace("{{ARCH}}", "x64")
$manifestPath = Join-Path $stage "AppxManifest.xml"
[IO.File]::WriteAllText($manifestPath, $manifest, [Text.UTF8Encoding]::new($false))

# valida XML antes de empacotar
[xml]$null = Get-Content $manifestPath -Raw

$makeappx = Find-SdkTool "makeappx.exe"
if (-not $makeappx) {
  throw "makeappx.exe nao encontrado. Instale o Windows SDK (Windows App Certification Kit) ou rode no CI windows-latest."
}

$outDir = Join-Path $root "dist"
New-Item -ItemType Directory -Path $outDir -Force | Out-Null
$msix = Join-Path $outDir "AVIFnim.Viewer_${displayVersion}_x64.msix"
if (Test-Path $msix) { Remove-Item $msix -Force }

Write-Host "==> makeappx pack"
& $makeappx pack /d $stage /p $msix /o
if ($LASTEXITCODE -ne 0) { throw "makeappx falhou ($LASTEXITCODE)" }

$signArgs = @()
if ($CertificateThumbprint) {
  $signArgs = @("sign", "/fd", "SHA256", "/sha1", $CertificateThumbprint, "/tr", $TimestampUrl, "/td", "SHA256", $msix)
} elseif ($CertificatePath) {
  if (-not (Test-Path $CertificatePath)) { throw "Certificado nao encontrado: $CertificatePath" }
  $signArgs = @("sign", "/fd", "SHA256", "/f", (Resolve-Path $CertificatePath), "/p", $CertificatePassword, "/tr", $TimestampUrl, "/td", "SHA256", $msix)
}

if ($signArgs.Count -gt 0) {
  $signtool = Find-SdkTool "signtool.exe"
  if (-not $signtool) { throw "signtool.exe nao encontrado (Windows SDK)." }
  Write-Host "==> signtool sign"
  & $signtool @signArgs
  if ($LASTEXITCODE -ne 0) { throw "signtool falhou ($LASTEXITCODE)" }
} else {
  Write-Warning "MSIX gerado SEM assinatura. Para testar localmente, assine com um certificado de code signing; para a Store, o Partner Center valida a identidade do pacote."
}

Write-Host ""
Write-Host "MSIX: $msix"
Write-Host ("Tamanho: {0:N2} MB" -f ((Get-Item $msix).Length / 1MB))
