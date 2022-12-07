use::one1fy::framework::*;

use one1fy::framework::components::*;

use one1fy::framework::components::bar::Orientation;

use skia_safe::{ Canvas, Rect, Color4f };
use skia_safe::paint::{ Paint };

use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;
use rand::Rng;

pub struct InputField {
    pub child: Rc<RefCell<dyn ComponentTraits>>,
    text: String,
    offer: String,
    // pub parent: Rc<RefCell<dyn ComponentTraits>>,
}

impl InputField {
    pub fn new(
        child: Rc<RefCell<dyn ComponentTraits>>,
        text: String,
    ) -> InputField {
        let x: String = String::new();
        InputField {
            child,
            text,
            offer: x,
        }
    }
}

impl Draw for InputField {
    fn draw(&mut self, canvas: &mut Canvas) {
        self.child.borrow_mut().draw(canvas);
    }
}

impl Find for InputField {
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

impl GetHeight for InputField {
    fn get_height(&self) -> u32 {
        self.child.borrow_mut().get_height()
    }
}

impl GetWidth for InputField {
    fn get_width(&self) -> u32 {
        self.child.borrow_mut().get_width()
    }
}

impl SetLeft for InputField {
    fn set_left(&mut self, val: u32) {
        self.child.borrow_mut().set_left(val);
    }
}

impl SetTop for InputField {
    fn set_top(&mut self, val: u32) {
        self.child.borrow_mut().set_top(val);
    }
}

impl GetType for InputField {
    fn get_type(&self) -> Option<Type> {
        Some(Type::TEXT)
    }
}

impl GetText for InputField {
    fn get_text(&mut self) -> Option<String> {
        Some(self.offer.clone())
    }
}

impl OnClick for InputField {
}

impl OnPress for InputField {
    fn on_press(&mut self, key: char) {
        if key == '~' {
            self.text.pop();
        }
        else if key == '$' {
            self.offer = self.text.clone();
            self.text.clear();
        }
        else {
            self.text.push(key)
        }
        let temp = self.text.clone();
        self.child.borrow_mut().set_text(temp);
    }
}

impl Remove for InputField {}

impl ToggleVisible for InputField {
    fn toggle_visible(&mut self) {
        self.child.borrow_mut().toggle_visible();
    }
}

impl SetStyle for InputField {
    fn set_style(&mut self, style: Style) {
        self.child.borrow_mut().set_style(style);
    }
}

impl SetText for InputField {
    fn set_text(&mut self, text: String) {
        self.offer.clear();
    }
}