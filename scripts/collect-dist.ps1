# Coleta os instaladores da v0.1.0 em ./dist
# - Windows: copia do build local (src-tauri/target/release/bundle/nsis)
# - Linux/macOS: baixa da release do GitHub (mesmos hashes publicados)
# Uso: powershell -NoProfile -ExecutionPolicy Bypass -File scripts/collect-dist.ps1
$ErrorActionPreference = 'Stop'

$root = Split-Path $PSScriptRoot -Parent
$dist = Join-Path $root 'dist'
New-Item -ItemType Directory -Force -Path $dist | Out-Null

$version = '0.1.0'
$releaseBase = "https://github.com/everctba/AVIFnim-Viewer/releases/download/v$version"

# Windows (build local)
$localNsis = Get-ChildItem (Join-Path $root 'src-tauri\target\release\bundle\nsis') -Filter '*.exe' -ErrorAction SilentlyContinue |
  Sort-Object LastWriteTime -Descending | Select-Object -First 1
if ($localNsis) {
  Copy-Item $localNsis.FullName (Join-Path $dist 'AVIFnim.Viewer_0.1.0_x64-setup.exe') -Force
  Write-Output ("windows: " + $localNsis.Name)
} else {
  Write-Output 'windows: build nao encontrado — rode npx tauri build --bundles nsis'
}

# Linux/macOS (download da release)
$remote = @(
  @{ Url = "$releaseBase/AVIFnim.Viewer_$version" + '_amd64.deb'; Name = "AVIFnim.Viewer_${version}_amd64.deb" },
  @{ Url = "$releaseBase/AVIFnim.Viewer_$version" + '_universal.dmg'; Name = "AVIFnim.Viewer_${version}_universal.dmg" }
)
foreach ($r in $remote) {
  $out = Join-Path $dist $r.Name
  if (-not (Test-Path $out)) {
    try {
      Invoke-WebRequest -Uri $r.Url -OutFile $out -UseBasicParsing -TimeoutSec 300
      Write-Output ("baixado: " + $r.Name)
    } catch {
      Write-Output ("falha ao baixar " + $r.Name + ": " + $_.Exception.Message)
    }
  } else {
    Write-Output ("ja existe: " + $r.Name)
  }
}

# Checksums
$lines = Get-ChildItem $dist -File | Where-Object { $_.Name -ne 'SHA256SUMS.txt' } | ForEach-Object {
  "{0}  {1}" -f (Get-FileHash $_.FullName -Algorithm SHA256).Hash.ToLower(), $_.Name
}
$lines | Set-Content (Join-Path $dist 'SHA256SUMS.txt') -Encoding ASCII
Write-Output 'dist pronto:'
Get-ChildItem $dist -File | Select-Object Name, Length | Format-Table -AutoSize
