@echo off
setlocal
cargo test
if errorlevel 1 exit /b %errorlevel%
echo.
echo Arcstone Execution Boundary local Rust verification PASS.
echo NOTE: OS-principal bypass tests are separate and are NOT proven by cargo test.
