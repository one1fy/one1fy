use::one1fy::framework::*;

use one1fy::framework::components::*;

use one1fy::framework::components::bar::Orientation;

use skia_safe::{ Canvas, Rect, Color4f };
use skia_safe::paint::{ Paint };

use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;



pub struct MyBox {
    pub child: Rc<RefCell<dyn ComponentTraits>>,
    pub link: String,
}

impl MyBox {
    pub fn new(
        child: Rc<RefCell<dyn ComponentTraits>>,
        link: String
    ) -> MyBox {
        MyBox {
            child,
            link,
        }
    }

    #[tokio::main]
    async fn send_request(&mut self, temp: String) -> Result<(), reqwest::Error> {
        let res = reqwest::get(temp.to_string()).await?;
        println!("HELLO WORLD");

        println!("Status: {}", res.status());

        let url = res.url();

        println!("URL: {}", url);

        open::that(temp.to_string()).unwrap();

        Ok(())
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

impl OnClick for MyBox {
    fn on_click(&mut self) {
        println!("PLEASE WORK");
        self.send_request(self.link.clone());
    }
}

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

impl GetText for MyBox {}

impl SetText for MyBox {}


#[cfg(any(feature = "windows", feature = "macos"))]
fn main() {
    build();
}


fn build() {

    let gen_height: u32 = 100;
    let get_width: u32 = 50;
    let gen_top: u32 = 100;

    let children: Vec<Box<dyn ComponentTraits>> = Vec::new();
    let mut bar: BarContainer = BarContainer::new(
        None,
        true,
        150,
        250,
        0,
        0,
        Some(children),
        Orientation::HORIZONTAL,
        true,
    );

    let box_style_1: Style = Style::new(
        Color::from_hex(0xffd500),
    );

    let mut box_1: BoxComponent = BoxComponent::new(
        62,
        gen_top,
        gen_height,
        get_width,
        box_style_1,
        true,
    );


    let box_style_2: Style = Style::new(
        Color::from_hex(0x000000),
    );

    let mut box_2: BoxComponent = BoxComponent::new(
        112,
        gen_top,
        gen_height,
        get_width,
        box_style_2,
        true,
    );


    let mut box_style_3: Style = Style::new(
        Color::from_hex(0xff0000),
    );
    
    let mut box_3: BoxComponent = BoxComponent::new(
        212,
        gen_top,
        gen_height,
        get_width,
        box_style_3,
        true,
    );

    let mut box_style_4: Style = Style::new(
        Color::from_hex(0xffffff),
    );
    
    let mut box_4: BoxComponent = BoxComponent::new(
        262,
        gen_top,
        gen_height,
        get_width,
        box_style_4,
        true,
    );

    

    let box_c = MyBox::new(Rc::new(RefCell::new(box_1)), "https://purdue.edu".to_string());
    let box_csharp = MyBox::new(Rc::new(RefCell::new(box_2)), "https://purdue.edu".to_string());
    let box_d = MyBox::new(Rc::new(RefCell::new(box_3)), "https://iu.edu".to_string());
    let box_e = MyBox::new(Rc::new(RefCell::new(box_4)), "https://iu.edu".to_string());
    

    bar.add_to_children(Box::new(box_c));
    bar.add_to_children(Box::new(box_csharp));
    bar.add_to_children(Box::new(box_d));
    bar.add_to_children(Box::new(box_e));
    

    run_app(bar);
}