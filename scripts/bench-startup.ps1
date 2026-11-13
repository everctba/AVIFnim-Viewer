# Benchmark de startup do AVIFnim Viewer
# Uso: powershell -File scripts\bench-startup.ps1 [-Runs 5]
param([int]$Runs = 5)

$exe = Join-Path $PSScriptRoot '..\src-tauri\target\release\avifnim-viewer.exe'
if (-not (Test-Path $exe)) {
  Write-Error "Build release nao encontrado. Rode: npx tauri build"
  exit 1
}

$results = @()
for ($i = 1; $i -le $Runs; $i++) {
  $sw = [Diagnostics.Stopwatch]::StartNew()
  $proc = Start-Process -FilePath $exe -PassThru
  $ok = $false
  while ($sw.Elapsed.TotalSeconds -lt 15) {
    $proc.Refresh()
    if ($proc.HasExited) { break }
    if ($proc.MainWindowHandle -ne [IntPtr]::Zero) { $ok = $true; break }
    Start-Sleep -Milliseconds 5
  }
  $sw.Stop()
  $ms = [math]::Round($sw.Elapsed.TotalMilliseconds)
  Write-Output ("run " + $i + ": " + $ms + " ms (window=" + $ok + ")")
  $results += $ms
  Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue
  Start-Sleep -Milliseconds 900
}

$avg = [math]::Round(($results | Measure-Object -Average).Average)
Write-Output ("startup avg: " + $avg + " ms | min " + ($results | Measure-Object -Minimum).Minimum + " ms | max " + ($results | Measure-Object -Maximum).Maximum + " ms")
