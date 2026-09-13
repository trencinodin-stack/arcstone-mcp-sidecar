@echo off
setlocal

if "%~1"=="" (
  echo Usage: %~nx0 ^<runtime-root^>
  exit /b 2
)

set "ROOT=%~1"
set "PROTECTED=%ROOT%\protected\effect.bin"
set "FORGED=%ROOT%\auth\issued\FORGED-BY-PRODUCER.json"

echo === Identity ===
whoami
echo.

echo === Attempt direct protected write ===
> "%PROTECTED%" echo PRODUCER_BYPASS_TEST
if errorlevel 1 (
  echo PASS: protected resource write was denied.
) else (
  echo FAIL: producer wrote the protected resource.
  exit /b 10
)

echo.
echo === Attempt forged authorization creation ===
> "%FORGED%" echo {}
if errorlevel 1 (
  echo PASS: authorization-store write was denied.
) else (
  echo FAIL: producer created authorization state.
  del "%FORGED%" >nul 2>&1
  exit /b 11
)

echo.
echo Producer authority-boundary checks PASS for the tested paths.
