use one1fy::framework::*;
use one1fy::framework::components::{
    BoxComponent,
    Style,
    Color,
};

#[cfg(feature = "windows")]
fn main() {
    build();
}

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