use one1fy::components::ComponentTraits;
use one1fy::framework::*;
use one1fy::framework::components::*;
use one1fy::framework::components::bar::Orientation;

// This function is only defined here because we are using windows.
// Otherwise, Swift or Andoird NDK will call build() directly.
#[cfg(any(feature = "windows", feature = "macos"))]
fn main() {
    build();
}


/// This must be defined always as this is the entry point into the user's code.
fn build() {

    let children: Vec<Box<dyn ComponentTraits>> = Vec::new();
    let mut bar: BarContainer = BarContainer::new(
        None,
        true,
        375,
        667,
        187,
        334,
        Some(children),
        Orientation::VERTICAL,
    );

    let box_style: Style = Style::new(
        Color::from_hex(0xff00ff),
    );

    let box_style_2: Style = Style::new(
        Color::from_hex(0x00ffff),
    );

    let mut red_box_1: BoxComponent = BoxComponent::new(
        0,
        0,
        100,
        100,
        box_style,
        true,
    );

    let mut red_box_2: BoxComponent = BoxComponent::new(
        0,
        0,
        100,
        100,
        box_style_2,
        true,
    );

    bar.add_to_children(Box::new(red_box_1));
    bar.add_to_children(Box::new(red_box_2));

    run_app(bar);
}
