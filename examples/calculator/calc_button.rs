use::one1fy::framework::*;

use one1fy::framework::components::*;

use one1fy::framework::components::bar::Orientation;

use skia_safe::{ Canvas, Rect, Color4f };
use skia_safe::paint::{ Paint };

use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;
use rand::Rng;

pub struct CalcButton {
    pub child: Rc<RefCell<dyn ComponentTraits>>,
    pub left: u32,
    pub top: u32,
    pub height: u32,
    pub width: u32,
    symbol: char,
    offer: bool,
}

impl CalcButton {
    pub fn new(
        child: Rc<RefCell<dyn ComponentTraits>>,
        left: u32,
        top: u32,
        height: u32,
        width: u32,
        symbol: char,
    ) -> CalcButton {
        CalcButton {
            child,
            left,
            top,
            height,
            width,
            symbol,
            offer: false,
        }
    }
}

impl Draw for CalcButton {
    fn draw(&mut self, canvas: &mut Canvas) {
        self.child.borrow_mut().draw(canvas);
        let mut box_symbol = TextComponent::new(
            self.left + self.width / 2,
            self.top + self.height / 2,
            self.width,
            self.height,
            20,
            true,
            self.symbol.to_string(),
            Color::from_hex(0x000000),
        );
        box_symbol.draw(canvas);

    }
}

impl Find for CalcButton {
    fn find(&mut self, x: u32, y: u32) -> Option<Uuid> {
        println!("searching...");
        let ret = self.child.borrow_mut().find(x,y);
        if let None = ret {
            None
        }
        else {
            self.on_click();
            ret
        }
        
    }
}

impl GetHeight for CalcButton {
    fn get_height(&self) -> u32 {
        self.child.borrow_mut().get_height()
    }
}

impl GetWidth for CalcButton {
    fn get_width(&self) -> u32 {
        self.child.borrow_mut().get_width()
    }
}

impl SetLeft for CalcButton {
    fn set_left(&mut self, val: u32) {
        self.child.borrow_mut().set_left(val);
    }
}

impl SetTop for CalcButton {
    fn set_top(&mut self, val: u32) {
        self.child.borrow_mut().set_top(val);
    }
}

impl GetType for CalcButton {
    fn get_type(&self) -> Option<Type> {
        Some(Type::BOX)
    }
}

impl GetText for CalcButton {
    fn get_text(&mut self) -> Option<String> {
        if self.offer {
            self.offer = false;
            return Some(self.symbol.to_string());
        }
        Some(String::new())
    }
}

impl OnClick for CalcButton {
    fn on_click(&mut self) {
        self.offer = true;
    }
}

impl OnPress for CalcButton {
    fn on_press(&mut self, key: char) {
        if key == self.symbol {
            self.offer = true;
        }
    }
}

impl Remove for CalcButton {}

impl ToggleVisible for CalcButton {
    fn toggle_visible(&mut self) {
        self.child.borrow_mut().toggle_visible();
    }
}

impl SetStyle for CalcButton {
    fn set_style(&mut self, style: Style) {
        self.child.borrow_mut().set_style(style);
    }
}

impl SetText for CalcButton {
}