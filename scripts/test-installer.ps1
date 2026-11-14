# Testa o instalador NSIS do AVIFnim Viewer: opt-in de associacoes + desinstalacao limpa.
# Requer o instalador gerado (npx tauri build --bundles nsis).
# Uso: powershell -NoProfile -ExecutionPolicy Bypass -File scripts/test-installer.ps1

$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
$bundleDir = Join-Path $root 'src-tauri\target\release\bundle\nsis'
$installDir = Join-Path $env:LOCALAPPDATA 'AVIFnim Viewer'
$testInstallDir = Join-Path $env:SystemDrive 'avifnim-installer-test'
$classes = 'HKCU:\Software\Classes'
$uninstallKey = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\AVIFnim Viewer'
$manuKey = 'HKCU:\Software\Everson Mayer\AVIFnim Viewer'

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

# Remove instalacao anterior (inclusive se o usuario escolheu outra pasta no instalador).
function Reset-InstallState {
  if (Test-Path $uninstallKey) {
    $location = $null
    try { $location = (Get-ItemProperty $uninstallKey -ErrorAction SilentlyContinue).InstallLocation } catch { $location = $null }
    if ($location) {
      $location = $location.Trim('"')
      $previousUninstaller = Join-Path $location 'uninstall.exe'
      if (Test-Path $previousUninstaller) {
        Start-Process -FilePath $previousUninstaller -ArgumentList '/S' -Wait
        Start-Sleep -Seconds 8
      }
      Remove-Item $location -Recurse -Force -ErrorAction SilentlyContinue
    }
    Remove-Item $uninstallKey -Recurse -Force -ErrorAction SilentlyContinue
  }
  Remove-Item $manuKey -Recurse -Force -ErrorAction SilentlyContinue
  foreach ($dir in @($installDir, $testInstallDir)) {
    if (Test-Path $dir) {
      $un = Join-Path $dir 'uninstall.exe'
      if (Test-Path $un) { Start-Process -FilePath $un -ArgumentList '/S' -Wait; Start-Sleep -Seconds 8 }
      Remove-Item $dir -Recurse -Force -ErrorAction SilentlyContinue
    }
  }
}

function Run-Case {
  param(
    [string]$Label,
    [string[]]$ExtraArgs,
    [hashtable]$Expected
  )
  Write-Output ("=== caso: $Label ===")

  # garante estado limpo (inclusive de instalacoes anteriores em outra pasta)
  Reset-InstallState

  # /D e obrigatorio ser o ultimo parametro e sem aspas (pasta de teste sem espacos)
  $extra = ($ExtraArgs -join ' ')
  $cmdLine = '"' + $setup.FullName + '" /S'
  if ($extra) { $cmdLine += " $extra" }
  $cmdLine += " /D=$testInstallDir"
  Start-Process -FilePath 'cmd.exe' -ArgumentList '/c', $cmdLine -Wait
  Start-Sleep -Seconds 3

  if (-not (Test-Path $testInstallDir)) {
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

  $uninstaller = Join-Path $testInstallDir 'uninstall.exe'
  if (-not (Test-Path $uninstaller)) {
    Write-Output '  FALHA: uninstall.exe ausente'
    $script:failures++
    return
  }
  Start-Process -FilePath $uninstaller -ArgumentList '/S' -Wait
  Start-Sleep -Seconds 8

  $dirGone = -not (Test-Path $testInstallDir)
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
