# Wrapper PowerShell do build (chama o build.bat, que configura o MSVC via vcvars64).
# Uso:
#   scripts\build.ps1 check
#   scripts\build.ps1 test
#   scripts\build.ps1 run
#   scripts\build.ps1 build --release
param(
  [Parameter(ValueFromRemainingArguments = $true)]
  [string[]]$Args
)

$bat = Join-Path $PSScriptRoot "build.bat"
if (-not (Test-Path $bat)) {
  Write-Error "build.bat nao encontrado em $PSScriptRoot"
  exit 1
}

$quoted = @()
foreach ($a in $Args) {
  if ($a -match '\s') { $quoted += '"' + $a + '"' } else { $quoted += $a }
}
$command = 'call "' + $bat + '" ' + ($quoted -join ' ')

cmd /c $command
exit $LASTEXITCODE
