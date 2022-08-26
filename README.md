# One1fy Platform
This repository holds the lowest level, platform specific code that the orchestrator interfaces with.

# Notable Dependencies
The need for this repository is to allow the orchestrator to have access to a Skia Canvas. From there,
the orchestrator can paint pixels onto the screen however the developer using the high level framework
desires. The current release only supports Windows architectures. This is due to issues with OpenGL on
Linux, and not enough research into MacOS.

# Use this Library
1) Use `cargo add one1fy-platform` in your source Rust project.
2) Ensure you have the clang compiler to build the Skia Dependencies.
3) Check the different features the `Cargo.toml` file has.
4) Use your given build target as a feature: `cargo build --features windows`
