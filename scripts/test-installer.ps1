# Testa o instalador NSIS do AVIFnim Viewer: opt-in de associacoes + desinstalacao limpa.
# Requer o instalador gerado (npx tauri build --bundles nsis).
# Uso: powershell -NoProfile -ExecutionPolicy Bypass -File scripts/test-installer.ps1

$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
$bundleDir = Join-Path $root 'src-tauri\target\release\bundle\nsis'
$installDir = Join-Path $env:LOCALAPPDATA 'AVIFnim Viewer'
$classes = 'HKCU:\Software\Classes'

$setup = Get-ChildItem $bundleDir -Filter '*.exe' -ErrorAction SilentlyContinue |
  Sort-Object LastWriteTime -Descending | Select-Object -First 1
if (-not $setup) {
  Write-Error "Instalador nao encontrado em $bundleDir. Rode: npx tauri build --bundles nsis"
  exit 1
}
Write-Output ("instalador: " + $setup.FullName)

function Get-AssocState {
  $state = [ordered]@{}
  foreach ($prog in @('avif', 'webp', 'gif', 'pnm')) {
    $state[$prog] = Test-Path (Join-Path $classes "AVIFnimViewer.$prog")
  }
  return $state
}

$failures = 0

function Run-Case {
  param(
    [string]$Label,
    [string[]]$ExtraArgs,
    [hashtable]$Expected
  )
  Write-Output ("=== caso: $Label ===")

  # garante estado limpo
  if (Test-Path $installDir) {
    $un = Join-Path $installDir 'uninstall.exe'
    if (Test-Path $un) { Start-Process -FilePath $un -ArgumentList '/S' -Wait; Start-Sleep -Seconds 6 }
    Remove-Item $installDir -Recurse -Force -ErrorAction SilentlyContinue
  }

  $argList = @('/S') + $ExtraArgs
  Start-Process -FilePath $setup.FullName -ArgumentList $argList -Wait
  Start-Sleep -Seconds 3

  if (-not (Test-Path $installDir)) {
    Write-Output '  FALHA: app nao instalou'
    $script:failures++
    return
  }

  $state = Get-AssocState
  $ok = $true
  foreach ($key in $Expected.Keys) {
    if ([bool]$state[$key] -ne [bool]$Expected[$key]) {
      Write-Output ("  DIVERGENCIA ${key}: obtido=$($state[$key]) esperado=$($Expected[$key])")
      $ok = $false
    }
  }
  if (-not $ok) { $script:failures++ }

  $uninstaller = Join-Path $installDir 'uninstall.exe'
  if (-not (Test-Path $uninstaller)) {
    Write-Output '  FALHA: uninstall.exe ausente'
    $script:failures++
    return
  }
  Start-Process -FilePath $uninstaller -ArgumentList '/S' -Wait
  Start-Sleep -Seconds 8

  $dirGone = -not (Test-Path $installDir)
  $after = Get-AssocState
  $cleaned = -not ($after.Values -contains $true)
  Write-Output ("  instalado=${ok} dir_removido=$dirGone associacoes_limpas=$cleaned")
  if (-not $dirGone -or -not $cleaned) { $script:failures++ }
}

Run-Case -Label 'sem /ASSOC (desmarcado por padrao)' -ExtraArgs @() -Expected @{ avif = $false; webp = $false; gif = $false; pnm = $false }
Run-Case -Label '/ASSOC=avif (somente AVIF)' -ExtraArgs @('/ASSOC=avif') -Expected @{ avif = $true; webp = $false; gif = $false; pnm = $false }
Run-Case -Label '/ASSOC (todos)' -ExtraArgs @('/ASSOC') -Expected @{ avif = $true; webp = $true; gif = $true; pnm = $true }

if ($failures -gt 0) {
  Write-Output ("TESTES DO INSTALADOR FALHARAM: $failures")
  exit 1
}
Write-Output 'TESTES DO INSTALADOR OK'
