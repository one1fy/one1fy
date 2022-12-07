use::one1fy::framework::*;

use one1fy::framework::components::*;

use one1fy::framework::components::bar::Orientation;
use one1fy::framework::components::checkbox::CheckType;

use skia_safe::{ Canvas, Rect, Color4f };
use skia_safe::paint::{ Paint };

use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;
use rand::Rng;

use crate::input;

pub struct ListBox {
    pub child: Rc<RefCell<dyn ComponentTraits>>,
    pub todos: Vec<Rc<RefCell<dyn ComponentTraits>>>,
    pub visible: bool,
    pub height: u32,
    pub width: u32,
    pub left: u32,
    pub top: u32,
}

impl ListBox {
    pub fn new(
        child: Rc<RefCell<dyn ComponentTraits>>,
        visible: bool,
        height: u32,
        width: u32,
        left: u32,
        top: u32,
    ) -> ListBox {
        let v = vec!();
        ListBox {
            child,
            todos: v,
            visible,
            height,
            width,
            left,
            top,
        }
    }
}

impl Draw for ListBox {
    fn draw(&mut self, canvas: &mut Canvas) {
        //check for new text
        let borrowed = self.child.borrow_mut().get_text().unwrap();
        if borrowed.len() > 0 {
            let cbox: CheckBox = CheckBox::new(
                0,
                0,
                33,
                100,
                true,
                CheckType::BOX,
                false,
                borrowed.clone(),
                Some(borrowed.clone()),
            );
            self.todos.push(Rc::new(RefCell::new(cbox)));
            self.child.borrow_mut().set_text("".to_string());
        }
        if self.visible {
            for child in self.todos.iter_mut() {
                child.borrow_mut().draw(canvas);
            }
            self.child.borrow_mut().draw(canvas);
        }
    }
}

impl Find for ListBox {
    fn find(&mut self, x: u32, y: u32) -> Option<Uuid> {
        for i in 0..self.todos.len() {
            let cur = &mut self.todos[i];
            let val: Option<Uuid> = cur.borrow_mut().find(x, y);
            if let None = val {
                continue;
            }
            else {
                return val;
            }
        }
        None
        
    }
}

impl GetHeight for ListBox {
    fn get_height(&self) -> u32 {
        self.height
    }
}

impl GetWidth for ListBox {
    fn get_width(&self) -> u32 {
        self.width
    }
}

impl SetLeft for ListBox {
    fn set_left(&mut self, val: u32) {
        self.left = val;
    }
}

impl SetTop for ListBox {
    fn set_top(&mut self, val: u32) {
        self.top = val;
    }
}

impl GetType for ListBox {
    fn get_type(&self) -> Option<Type> {
        Some(Type::CONTAINER)
    }
}

impl GetText for ListBox {
}

impl OnClick for ListBox {
}

impl OnPress for ListBox {
    fn on_press(&mut self, key: char) {
        self.child.borrow_mut().on_press(key);
    }
}

impl Remove for ListBox {}

impl ToggleVisible for ListBox {
    fn toggle_visible(&mut self) {
        self.visible = !self.visible;
    }
}

impl SetStyle for ListBox {
}

impl SetText for ListBox {
}