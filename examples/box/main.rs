use one1fy::framework::*;
use one1fy::framework::components::{
    BoxComponent,
    Style,
};

#[cfg(feature = "windows")]
fn main() {
    build();
}

fn build() {
    let box_style: Style = Style::new(0xff0000);

    let red_box: BoxComponent = BoxComponent::new(0, 0, 100, 100, box_style);

    run_app(red_box);
}