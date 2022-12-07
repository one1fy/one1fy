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
    let text: String = "hello world".to_string();
    let color: Color = Color::from_hex(0xFF0000);
    let component: TextComponent = TextComponent::new(100, 100, 10, 100, 100, true, text, color);

    let children: Vec<Box<dyn ComponentTraits>> = Vec::new();
    let mut bar: BarContainer = BarContainer::new(
        None,
        true,
        375,
        667,
        0,
        0,
        Some(children),
        Orientation::HORIZONTAL,
        false,
    );
    bar.add_to_children(Box::new(component));

    run_app(bar);
}
