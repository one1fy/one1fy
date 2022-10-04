use::one1fy::framework::*;

use one1fy::framework::components::{
    BoxComponent,
    Style,
    Color,
    BarContainer,
}

#[cfg(any(feature = "windows", feature = "macos"))]
fn main() {
    build();
}

fn build() {
    let box_style_1: Style = Style::new(
        Color::from_hex(0xff00ff),
    );

    let red_box_1: BoxComponent = BoxComponent::new(
        0,
        0,
        100,
        100,
        box_style,
    );

    let box_style_2: Style = Style::new(
        Color::from_hex(0xff00ff),
    );

    let red_box_2: BoxComponent = BoxComponent::new(
        0,
        0,
        100,
        100,
        box_style,
    );

    let box_style_3: Style = Style::new(
        Color::from_hex(0xff00ff),
    );

    let red_box_3: BoxComponent = BoxComponent::new(
        0,
        0,
        100,
        100,
        box_style,
    );
    let children: Vec<Box<dyn draw>> = Vec::new();
    bar.add_to_children(red_box_1);
    bar.add_to_children(red_box_2);
    bar.add_to_children(red_box_3);

    let bar: BarContainer = BarContainer::new(
        None,
        true,
        500,
        500,
        0,
        0,
        children,
        Orientation::HORIZONTAL,
    )

    run_app(bar);
}