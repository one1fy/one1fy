use::one1fy::framework::*;

use one1fy::framework::components::*;

use one1fy::framework::components::bar::Orientation;

use skia_safe::{ Canvas, Rect, Color4f };
use skia_safe::paint::{ Paint };

use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;
use rand::Rng;

mod input;
mod list;



#[cfg(any(feature = "windows", feature = "macos"))]
fn main() {
    build();
}


fn build() {

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
        true,
    );

    let text: String = "hello world".to_string();
    let color: Color = Color::from_hex(0x000000);
    let component: TextComponent = TextComponent::new(105, 1055, 100, 500, 20, true, text.clone(), color);

    let input = input::InputField::new(Rc::new(RefCell::new(component)), text.clone());
    let list = list::ListBox::new(Rc::new(RefCell::new(input)), true, 100, 100, 0, 0);
    bar.add_to_children(Box::new(list));

    run_app(bar);
}