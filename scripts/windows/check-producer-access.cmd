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

echo === Preconditions ===
if exist "%PROTECTED%" (
  echo ERROR: protected target already exists; refusing ambiguous T7 test.
  exit /b 20
)

if exist "%FORGED%" (
  echo ERROR: forged-authorization probe already exists; refusing ambiguous T9 test.
  exit /b 21
)

echo Preconditions PASS.
echo.

echo === Attempt direct protected write ===
> "%PROTECTED%" echo PRODUCER_BYPASS_TEST

if exist "%PROTECTED%" (
  echo FAIL: producer wrote the protected resource.
  exit /b 10
) else (
  echo PASS: protected resource write was denied.
)

echo.
echo === Attempt forged authorization creation ===
> "%FORGED%" echo {}

if exist "%FORGED%" (
  echo FAIL: producer created authorization state.
  exit /b 11
) else (
  echo PASS: authorization-store write was denied.
)

echo.
echo Producer authority-boundary checks PASS for the tested paths.
exit /b 0
