use::one1fy::framework::*;

use one1fy::framework::components::*;

use one1fy::framework::components::bar::Orientation;

use skia_safe::{ Canvas, Rect, Color4f };
use skia_safe::paint::{ Paint };

use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;
use rand::Rng;

pub struct MyBox {
    pub child: Rc<RefCell<dyn ComponentTraits>>,
    // pub parent: Rc<RefCell<dyn ComponentTraits>>,
}

impl MyBox {
    pub fn new(
        child: Rc<RefCell<dyn ComponentTraits>>,
        //parent: Rc<RefCell<dyn ComponentTraits>>,
    ) -> MyBox {
        MyBox {
            child,
            //parent,
        }
    }
}

impl Draw for MyBox {
    fn draw(&mut self, canvas: &mut Canvas) {
        self.child.borrow_mut().draw(canvas);
    }
}

impl Find for MyBox {
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

impl GetHeight for MyBox {
    fn get_height(&self) -> u32 {
        self.child.borrow_mut().get_height()
    }
}

impl GetWidth for MyBox {
    fn get_width(&self) -> u32 {
        self.child.borrow_mut().get_width()
    }
}

impl SetLeft for MyBox {
    fn set_left(&mut self, val: u32) {
        self.child.borrow_mut().set_left(val);
    }
}

impl SetTop for MyBox {
    fn set_top(&mut self, val: u32) {
        self.child.borrow_mut().set_top(val);
    }
}

impl GetType for MyBox {
    fn get_type(&self) -> Option<Type> {
        Some(Type::BOX)
    }
}

impl GetText for MyBox {}

impl OnClick for MyBox {
    fn on_click(&mut self) {
        let num = rand::thread_rng().gen_range(0..0xffffffff);
        let style: Style = Style::new(Color::from_hex(num));
        self.child.borrow_mut().set_style(style);
    }
}

impl OnPress for MyBox {}

impl Remove for MyBox {}

impl ToggleVisible for MyBox {
    fn toggle_visible(&mut self) {
        self.child.borrow_mut().toggle_visible();
    }
}

impl SetStyle for MyBox {
    fn set_style(&mut self, style: Style) {
        self.child.borrow_mut().set_style(style);
    }
}

impl SetText for MyBox {}


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
        false,
    );

    let box_style_1: Style = Style::new(
        Color::from_hex(0xff00ff),
    );

    
    let mut red_box_1: BoxComponent = BoxComponent::new(
        0,
        0,
        100,
        100,
        box_style_1,
        true,
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
    );

    let mb1 = MyBox::new(Rc::new(RefCell::new(red_box_1)));
    let mb2 = MyBox::new(Rc::new(RefCell::new(red_box_2)));
    let mb3 = MyBox::new(Rc::new(RefCell::new(red_box_3)));

    bar.add_to_children(Box::new(mb1));
    bar.add_to_children(Box::new(mb2));
    bar.add_to_children(Box::new(mb3));

    run_app(bar);
}