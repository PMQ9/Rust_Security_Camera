# Rust_Security_Camera
Test projects to learn Rust
- Build a camera surveillance system to protect an area.
- Computer vision to determines suspicious activities.
- Log activites. If no suspicious activity, delete footage.
- Use LED blinking in a secure, random-generated pattern. If the pattern is not correct, then the visual feedback is tampered.
- LED blinker can be from the Pi5 or the keyboard CapsLock light.

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


