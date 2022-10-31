use::one1fy::framework::*;

use one1fy::framework::components::*;

use one1fy::framework::components::bar::Orientation;

use uuid::Uuid;


#[cfg(any(feature = "windows", feature = "macos"))]
fn main() {
    build();
}


fn build() {
    let box_style_1: Style = Style::new(
        Color::from_hex(0xff00ff),
    );

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
    );

    // let mut onCl = |id: Uuid| {
    //     bar.remove(id);
    // };

    
    let mut red_box_1: BoxComponent = BoxComponent::new(
        0,
        0,
        100,
        100,
        box_style_1,
        true,
        Box::new(|&mut self| {
            self.visible = false;
        }),
    );

    let box_style_2: Style = Style::new(
        Color::from_hex(0x00ffff),
    );

    let mut red_box_2: BoxComponent = BoxComponent::new(
        0,
        0,
        100,
        100,
        box_style_2,
        true,
        Box::new(|&mut self| {
            self.visible = false;
        }),
    );

    let mut box_style_3: Style = Style::new(
        Color::from_hex(0x0000ff),
    );
    

    let mut red_box_3: BoxComponent = BoxComponent::new(
        0,
        0,
        100,
        100,
        box_style_3,
        true,
        Box::new(|&mut self| {
            self.visible = false;
        }),
    );

    

    bar.add_to_children(Box::new(red_box_1));
    bar.add_to_children(Box::new(red_box_2));
    bar.add_to_children(Box::new(red_box_3));

    run_app(bar);
}
