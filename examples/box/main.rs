use one1fy::framework::*;
use one1fy::framework::components::{
    BoxComponent,
    Style,
    Color,
};

// This function is only defined here because we are using windows.
// Otherwise, Swift or Andoird NDK will call build() directly.
#[cfg(feature = "windows")]
fn main() {
    build();
}

/// This must be defined always as this is the entry point into the user's code.
fn build() {
    let box_style: Style = Style::new(
        Color::from_hex(0xff00ff),
    );

    let red_box: BoxComponent = BoxComponent::new(
        0.0,
        0.0,
        100.0,
        100.0,
        box_style,
    );

    run_app(red_box);
}