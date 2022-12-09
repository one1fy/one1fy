use::one1fy::framework::*;

use one1fy::framework::components::*;

use one1fy::framework::components::bar::Orientation;

use skia_safe::{ Canvas, Rect, Color4f };
use skia_safe::paint::{ Paint };

use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;
use rand::Rng;

mod calc_button;
mod screen;



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

    //styling
    let text = String::new();
    let color: Color = Color::from_hex(0xff00ff);
    let boxStyle = Style::new(
        Color::from_hex(0xd1d0c3)
    );

    //primitives
    let component: TextComponent = TextComponent::new(115, 230, 100, 500, 20, true, text.clone(), color);
    let button_box_1 = BoxComponent::new(115, 275, 75, 75, boxStyle, true);
    let button_box_4 = BoxComponent::new(115, 375, 75, 75, boxStyle, true);
    let button_box_7 = BoxComponent::new(115, 475, 75, 75, boxStyle, true);
    let button_box_0 = BoxComponent::new(115, 575, 75, 75, boxStyle, true);

    let button_box_2 = BoxComponent::new(215, 275, 75, 75, boxStyle, true);
    let button_box_5 = BoxComponent::new(215, 375, 75, 75, boxStyle, true);
    let button_box_8 = BoxComponent::new(215, 475, 75, 75, boxStyle, true);

    let button_box_3 = BoxComponent::new(315, 275, 75, 75, boxStyle, true);
    let button_box_6 = BoxComponent::new(315, 375, 75, 75, boxStyle, true);
    let button_box_9 = BoxComponent::new(315, 475, 75, 75, boxStyle, true);

    let button_box_plus = BoxComponent::new(415, 175, 75, 75, boxStyle, true);
    let button_box_div = BoxComponent::new(415, 275, 75, 75, boxStyle, true);
    let button_box_mult = BoxComponent::new(415, 375, 75, 75, boxStyle, true);
    let button_box_min = BoxComponent::new(415, 475, 75, 75, boxStyle, true);
    let button_box_eq = BoxComponent::new(415, 575, 75, 75, boxStyle, true);

    //wrappers
    let button_1 = calc_button::CalcButton::new(Rc::new(RefCell::new(button_box_1)), 100, 300, 75, 75, '1');
    let button_4 = calc_button::CalcButton::new(Rc::new(RefCell::new(button_box_4)), 100, 400, 75, 75, '4');
    let button_7 = calc_button::CalcButton::new(Rc::new(RefCell::new(button_box_7)), 100, 500, 75, 75, '7');
    let button_0 = calc_button::CalcButton::new(Rc::new(RefCell::new(button_box_0)), 100, 600, 75, 75, '0');

    let button_2 = calc_button::CalcButton::new(Rc::new(RefCell::new(button_box_2)), 200, 300, 75, 75, '2');
    let button_5 = calc_button::CalcButton::new(Rc::new(RefCell::new(button_box_5)), 200, 400, 75, 75, '5');
    let button_8 = calc_button::CalcButton::new(Rc::new(RefCell::new(button_box_8)), 200, 500, 75, 75, '8');

    let button_3 = calc_button::CalcButton::new(Rc::new(RefCell::new(button_box_3)), 300, 300, 75, 75, '3');
    let button_6 = calc_button::CalcButton::new(Rc::new(RefCell::new(button_box_6)), 300, 400, 75, 75, '6');
    let button_9 = calc_button::CalcButton::new(Rc::new(RefCell::new(button_box_9)), 300, 500, 75, 75, '9');

    let button_plus = calc_button::CalcButton::new(Rc::new(RefCell::new(button_box_plus)), 400, 200, 75, 75, '+');
    let button_div = calc_button::CalcButton::new(Rc::new(RefCell::new(button_box_div)), 400, 300, 75, 75, '/');
    let button_mult = calc_button::CalcButton::new(Rc::new(RefCell::new(button_box_mult)), 400, 400, 75, 75, '*');
    let button_min = calc_button::CalcButton::new(Rc::new(RefCell::new(button_box_min)), 400, 500, 75, 75, '-');
    let button_eq = calc_button::CalcButton::new(Rc::new(RefCell::new(button_box_eq)), 400, 600, 75, 75, '=');

    let mut buttons = Vec::new();
    buttons.push(Rc::new(RefCell::new(button_1)));
    buttons.push(Rc::new(RefCell::new(button_4)));
    buttons.push(Rc::new(RefCell::new(button_7)));
    buttons.push(Rc::new(RefCell::new(button_2)));
    buttons.push(Rc::new(RefCell::new(button_5)));
    buttons.push(Rc::new(RefCell::new(button_8)));
    buttons.push(Rc::new(RefCell::new(button_3)));
    buttons.push(Rc::new(RefCell::new(button_6)));
    buttons.push(Rc::new(RefCell::new(button_9)));
    buttons.push(Rc::new(RefCell::new(button_plus)));
    buttons.push(Rc::new(RefCell::new(button_div)));
    buttons.push(Rc::new(RefCell::new(button_mult)));
    buttons.push(Rc::new(RefCell::new(button_min)));
    buttons.push(Rc::new(RefCell::new(button_eq)));
    buttons.push(Rc::new(RefCell::new(button_0)));
    let screen = screen::CalcScreen::new(
        Rc::new(RefCell::new(component)),
        buttons,
        105,
        1055,
        100,
        500,
        true,
    );

    bar.add_to_children(Box::new(screen));




    run_app(bar);
}