@echo off
echo Copying DLL files...
xcopy /Y /I "D:\Projects\vcpkg\installed\x64-windows\bin\*.dll" "D:\Projects\Rust_Security_Camera\target\debug\"
echo DLL files copied successfully!
pause