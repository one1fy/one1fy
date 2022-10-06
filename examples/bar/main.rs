use::one1fy::framework::*;

use one1fy::framework::components::*;

use one1fy::framework::components::bar::Orientation;


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
        box_style_1,
        true,
    );

    let box_style_2: Style = Style::new(
        Color::from_hex(0x00ffff),
    );

    let red_box_2: BoxComponent = BoxComponent::new(
        0,
        0,
        100,
        100,
        box_style_2,
        true,
    );

    let box_style_3: Style = Style::new(
        Color::from_hex(0x0000ff),
    );

    let red_box_3: BoxComponent = BoxComponent::new(
        0,
        0,
        100,
        100,
        box_style_3,
        true,
    );
    let children: Vec<Box<dyn Component_Traits>> = Vec::new();
    fn nothing() {}
    let mut bar: BarContainer = BarContainer::new(
        nothing,
        true,
        375,
        667,
        0,
        0,
        children,
        Orientation::HORIZONTAL,
    );

    bar.add_to_children(Box::new(red_box_1));
    bar.add_to_children(Box::new(red_box_2));
    bar.add_to_children(Box::new(red_box_3));

    run_app(bar);
}