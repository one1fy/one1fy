# One1fy
A mobile framework to rule them all. One1fy aims to be mobile platform
agnostic and language agnostic. Currently we only support Rust

# Supported Platforms
* Windows

# Supported Languages
* Rust

# Notable Dependencies
Skia is the graphics library being used for this project.
Skia will allow the framework defined components to be drawn onto a canvas.

# Use this Library
1) Clone the repository
2) Ensure you have the clang compiler to build the Skia Dependencies.
3) Check the different features the `Cargo.toml` file has.
4) Use your given build target as a feature: `cargo build --features windows`

# Examples
* box - Displays a single box onto the screen