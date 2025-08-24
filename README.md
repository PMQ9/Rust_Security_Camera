# Rust_Security_Camera
Test projects to learn Rust
- Build a camera surveillance system to protect an area.
- Computer vision to determines suspicious activities.
- Log activites. If no suspicious activity, delete footage.
- Use LED blinking in a secure, random-generated pattern. If the pattern is not correct, then the visual feedback is tampered.
- LED blinker can be from the Pi5 or the keyboard CapsLock light.

# Requirements
1. Install Rust
2. Install OpenCV: The Rust `opencv` crate requires the OpenCV library to be installed
    - Use version 4.9.0
    - Add OpenCV binaries to system's PATH environment variable
    - Set environment variables: `OPENCV_DIR` to `C:\opencv\build` and `PKG_CONFIG_PATH` to `C:\opencv\build\x64\vc16\lib`, might assist with crate compilation.
3. Install LLVM `https://releases.llvm.org/download.html`
    - Use version 15.0.7 `LLVM-15.0.7-win64.exe`.
    - Add LLVM binaries `C:\Program Files\LLVM\bin` to system's PATH environment variable. 
    - Set environment variables: `LIBCLANG_PATH` to `C:\Program Files\LLVM\bin`.
4. Set temporary variables:
    - `set VCPKG_ROOT=C:\vcpkg`
    - `set OPENCV_DIR=C:\vcpkg\installed\x64-windows`


