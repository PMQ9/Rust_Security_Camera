# Rust_Security_Camera
Projects to learn Rust
- Build a camera surveillance system to protect an area.
- Computer vision to determines suspicious activities.
- Use LED blinking in a secure, random-generated pattern. If the pattern is not correct, then the visual feedback is tampered.

# Requirements
1. Microsoft C++ Build Tools
2. Cmake
3. Rust
4. OpenCV: The Rust `opencv` crate requires the OpenCV library to be installed
    - Use version 4.10.0
5. Install LLVM `https://releases.llvm.org/download.html`
    - Use version 19.1.3
6. If you are using Windows, add the following Environment Variables:
    - `OPENCV_LINK_LIBS` = `"opencv_core4,opencv_highgui4,opencv_imgproc4,opencv_videoio4"`
    - `OPENCV_LINK_PATHS` = `"C:\...\vcpkg\installed\x64-windows\lib"`
    - `OPENCV_INCLUDE_PATHS` = `"C:\...\vcpkg\installed\x64-windows\include\opencv4"`
    - `OPENCV_VERSION` = `"4.11.0"`
    - `OPENCV_DISABLE_PROBES` = `"cmake,vcpkg_cmake,vcpkg,pkg_config"`
    - `OPENCV_PACKAGE_NAME` = `"opencv4"`
    - `OPENCV_VCPKG_NAME` = `"opencv4"`
    - `VCPKG_ROOT` = `"C:\...\vcpkg"`
    - `VCPKGRS_DYNAMIC` = `"1"`
    - `CMAKE_PREFIX_PATH` = `"C:\...\vcpkg\installed\x64-windows"`

# Instruction

1. Start Security Pattern on Raspberry Pi4
    - `cd src/controller/led`
    - `rustc led_controller.rs`
    - `sudo ./led_controller`
2. Compile Rust_Security_Camera
    - `cargo clean`
    - `cargo build`
    - Might need to run this script to add some missing libraries: `.\utils\copy_dll_files_from_vcpkg_to_target.bat`
3. Wait for the program to self calibrate, please ensure consistent lighting for the best result

# Demo

https://github.com/user-attachments/assets/1a495fa7-ec8a-432a-807a-399714ec5916

Raw: https://github.com/PMQ9/Rust_Security_Camera/blob/main/doc/2025-08-29%2020-05-12.mp4




