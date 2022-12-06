use::one1fy::framework::*;

use one1fy::framework::components::*;

use one1fy::framework::components::bar::Orientation;
use one1fy::framework::components::checkbox::CheckType;

use skia_safe::{ Canvas, Rect, Color4f };
use skia_safe::paint::{ Paint };

use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;
use rand::Rng;

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
        187,
        334,
        Some(children),
        Orientation::VERTICAL,
    );

    let cbox: CheckBox = CheckBox::new(
        0,
        0,
        33,
        100,
        true,
        CheckType::BOX,
        false,
        "I am cool".to_string(),
        Some("I am not cool".to_string()),
    );

    let cbox2: CheckBox = CheckBox::new(
        0,
        0,
        33,
        100,
        true,
        CheckType::BOX,
        false,
        "One1fy is awesome".to_string(),
        Some("One1fy is still awesome".to_string()),
    );

    let cbox3: CheckBox = CheckBox::new(
        0,
        0,
        33,
        100,
        true,
        CheckType::BOX,
        false,
        "I am Dom".to_string(),
        Some("I am not Dom".to_string()),
    );

    bar.add_to_children(Box::new(cbox));
    bar.add_to_children(Box::new(cbox3));
    bar.add_to_children(Box::new(cbox2));
    

    run_app(bar);
}