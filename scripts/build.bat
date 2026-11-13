@echo off
rem Localiza o vcvars64.bat via vswhere e executa cargo com o ambiente do MSVC.
rem Uso: scripts\build.ps1 <args do cargo>
setlocal

set "VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"
if not exist "%VSWHERE%" (
  echo [erro] vswhere.exe nao encontrado. Instale o Visual Studio Build Tools com C++.
  exit /b 1
)

for /f "usebackq delims=" %%i in (`"%VSWHERE%" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do set "VSPATH=%%i"

if not defined VSPATH (
  echo [erro] Build Tools com MSVC nao encontrado.
  exit /b 1
)

call "%VSPATH%\VC\Auxiliary\Build\vcvars64.bat" >nul
if errorlevel 1 (
  echo [erro] falha ao configurar o ambiente do MSVC.
  exit /b 1
)

cd /d "%~dp0..\src-tauri"
cargo %*
