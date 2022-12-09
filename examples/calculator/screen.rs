use::one1fy::framework::*;

use one1fy::framework::components::*;

use one1fy::framework::components::bar::Orientation;
use one1fy::framework::components::checkbox::CheckType;

use skia_safe::{ Canvas, Rect, Color4f };
use skia_safe::paint::{ Paint };

use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;
use rand::Rng;

use crate::calc_button;

pub struct CalcScreen {
    pub child: Rc<RefCell<dyn ComponentTraits>>,
    pub buttons: Vec<Rc<RefCell<calc_button::CalcButton>>>,
    pub operand: Option<i32>,
    pub operation: char,
    pub left: u32,
    pub top: u32,
    pub height: u32,
    pub width: u32,
    pub text: String,
    pub numString: String,
    pub visible: bool,
}

impl CalcScreen {
    pub fn new(
        child: Rc<RefCell<dyn ComponentTraits>>,
        buttons: Vec<Rc<RefCell<calc_button::CalcButton>>>,
        left: u32,
        top: u32,
        height: u32,
        width: u32,
        visible: bool,
        
    ) -> CalcScreen {
        CalcScreen {
            child,
            buttons,
            operand: None,
            operation: 'x',
            left,
            top,
            height,
            width,
            text: String::new(),
            numString: String::new(),
            visible,
        }
    }
    pub fn calc(&mut self) {
        let operands = vec!['+', '-', '*', '/', '=', 'µ'];
        for i in 0..self.buttons.len() {
            let cur = &mut self.buttons[i];
            let val: String = cur.borrow_mut().get_text().unwrap();
            if val.len() > 0 {
                let c = val.chars().next().unwrap();
                if operands.contains(&c) {
                    println!("operand found");
                    if self.numString.len() == 0 {
                        self.throw_err();
                        break;
                    }
                    let num = self.numString.parse::<i32>().unwrap();
                    if self.operand.is_none() {
                        self.operand = Some(num);
                    }

                    if self.operation == 'x' {
                        self.operation = c;
                    }
                    else if self.operation == '+' {
                        self.operand = Some(self.operand.unwrap() + num);
                    }
                    else if self.operation == '-' {
                        self.operand = Some(self.operand.unwrap() - num);
                    }
                    else if self.operation == '*' {
                        self.operand = Some(self.operand.unwrap() * num);
                    }
                    else if self.operation == '/' {
                        self.operand = Some(self.operand.unwrap() / num);
                    }
                    else {

                    }

                    if c == '=' || c == 'µ' {
                        if !self.operand.is_none() {
                            self.text = self.operand.unwrap().to_string();
                            self.numString.clear();
                        } 
                        self.operation = 'x';
                        self.operand = None;
                    }
                    else {
                        self.operation = c;
                        self.text = c.to_string();
                        self.numString.clear();
                    }
                }
                else {
                    self.numString.push(c);
                    self.text = self.numString.clone();
                }
                break;
            }
        }
    }

    pub fn throw_err(&mut self) {
        self.text = "err!".to_string();
        self.operation = 'x';
        self.operand = None;
    }
}

impl Draw for CalcScreen {
    fn draw(&mut self, canvas: &mut Canvas) {
        //calculate values
        self.calc();
        self.child.borrow_mut().set_text(self.text.clone());
        if self.visible {
            let yellow = Style::new(
                Color::from_hex(0x78dfe2),
            );
            let white = Style::new(
                Color::from_hex(0xffffff),
            );
            let mut background = BoxComponent::new(0, 0, 2000, 2000, yellow, true);
            background.draw(canvas);
            let mut input_box = BoxComponent::new(115, 175, 75, 275, white, true);
            input_box.draw(canvas);
            let mut title = TextComponent::new(200, 100, 150, 500, 100, true, "Calculator".to_string(), Color::from_hex(0x000000));
            title.draw(canvas);
            for child in self.buttons.iter_mut() {
                child.borrow_mut().draw(canvas);
            }
            self.child.borrow_mut().draw(canvas);
        }
    }
}

impl Find for CalcScreen {
    fn find(&mut self, x: u32, y: u32) -> Option<Uuid> {
        for i in 0..self.buttons.len() {
            let cur = &mut self.buttons[i];
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

impl GetHeight for CalcScreen {
    fn get_height(&self) -> u32 {
        self.height
    }
}

impl GetWidth for CalcScreen {
    fn get_width(&self) -> u32 {
        self.width
    }
}

impl SetLeft for CalcScreen {
    fn set_left(&mut self, val: u32) {
        self.left = val;
    }
}

impl SetTop for CalcScreen {
    fn set_top(&mut self, val: u32) {
        self.top = val;
    }
}

impl GetType for CalcScreen {
    fn get_type(&self) -> Option<Type> {
        Some(Type::CONTAINER)
    }
}

impl GetText for CalcScreen {
    fn get_text(&mut self) -> Option<String> {
        Some(self.text.clone())
    }
}

impl OnClick for CalcScreen {

}

impl OnPress for CalcScreen {
    fn on_press(&mut self, key: char) {
        for i in 0..self.buttons.len() {
            let cur = &mut self.buttons[i];
            cur.borrow_mut().on_press(key);
        }
    }
}

impl Remove for CalcScreen {}

impl ToggleVisible for CalcScreen {
    fn toggle_visible(&mut self) {
        self.visible = !self.visible;
    }
}

impl SetStyle for CalcScreen {
}

impl SetText for CalcScreen {
}