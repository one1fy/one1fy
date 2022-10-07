use one1fy::components::Component_Traits;
use one1fy::framework::*;
use one1fy::framework::components::{
    BoxComponent,
    Style,
    Color,
    TextComponent,
};

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
    let component: TextComponent = TextComponent::new(0, 0, 10, text, color);
    let tree: Box<dyn Component_Traits> = Box::new(component);

    run_app(tree);
}
