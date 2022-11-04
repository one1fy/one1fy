use::one1fy::framework::*;

use one1fy::framework::components::*;

use one1fy::framework::components::bar::Orientation;

use skia_safe::{ Canvas, Rect, Color4f };
use skia_safe::paint::{ Paint };

use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;
use rand::Rng;

use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

pub struct MyBox {
    pub child: Rc<RefCell<dyn ComponentTraits>>,
    pub tone: f32,
    // pub parent: Rc<RefCell<dyn ComponentTraits>>,
}

impl MyBox {
    pub fn new(
        child: Rc<RefCell<dyn ComponentTraits>>,
        tone: f32
        //parent: Rc<RefCell<dyn ComponentTraits>>,
    ) -> MyBox {
        MyBox {
            child,
            tone,
            //parent,
        }
    }

    pub fn play_tone(&mut self, note: f32) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Add a dummy source of the sake of the example.
        let source_A = SineWave::new(note).take_duration(Duration::from_secs_f32(0.5)).amplify(0.20);
        sink.append(source_A);

        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        sink.sleep_until_end();
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
        let num = rand::thread_rng().gen_range(0..0xffffffff);
        let style: Style = Style::new(Color::from_hex(num));
        //self.child.borrow_mut().set_style(style);
        self.play_tone(self.tone);
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

    pub struct Note;

    impl Note {
        pub const C: f32 = 523.25;
        pub const Csharp: f32 = 554.37;
        pub const D: f32 = 587.33;
        pub const Eflat: f32 = 622.25;
        pub const E: f32 = 659.25;
        pub const F: f32 = 698.46;
        pub const Fsharp: f32 = 739.99;
        pub const G: f32 = 783.99;
        pub const Aflat: f32 = 830.61;
        pub const A: f32 = 880.00;
        pub const Bflat: f32 = 932.33;
        pub const B: f32 = 987.77;
    }


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
    );

    let box_style_1: Style = Style::new(
        Color::from_hex(0x9400d3),
    );

    let mut box_1: BoxComponent = BoxComponent::new(
        0,
        0,
        100,
        50,
        box_style_1,
        true,
    );


    let box_style_2: Style = Style::new(
        Color::from_hex(0x0000ff),
    );

    let mut box_2: BoxComponent = BoxComponent::new(
        0,
        0,
        100,
        50,
        box_style_2,
        true,
    );


    let mut box_style_3: Style = Style::new(
        Color::from_hex(0x00ff00),
    );
    
    let mut box_3: BoxComponent = BoxComponent::new(
        0,
        0,
        100,
        50,
        box_style_3,
        true,
    );


    let mut box_style_4: Style = Style::new(
        Color::from_hex(0xffff00),
    );
    
    let mut box_4: BoxComponent = BoxComponent::new(
        0,
        0,
        100,
        50,
        box_style_4,
        true,
    );


    let mut box_style_5: Style = Style::new(
        Color::from_hex(0xff0000),
    );
    
    let mut box_5: BoxComponent = BoxComponent::new(
        0,
        0,
        100,
        50,
        box_style_5,
        true,
    );


    let mb1 = MyBox::new(Rc::new(RefCell::new(box_1)), Note::C);
    let mb2 = MyBox::new(Rc::new(RefCell::new(box_2)), Note::D);
    let mb3 = MyBox::new(Rc::new(RefCell::new(box_3)), Note::E);
    let mb4 = MyBox::new(Rc::new(RefCell::new(box_4)), Note::F);
    let mb5 = MyBox::new(Rc::new(RefCell::new(box_5)), Note::G);

    bar.add_to_children(Box::new(mb1));
    bar.add_to_children(Box::new(mb2));
    bar.add_to_children(Box::new(mb3));
    bar.add_to_children(Box::new(mb4));
    bar.add_to_children(Box::new(mb5));

    run_app(bar);
}