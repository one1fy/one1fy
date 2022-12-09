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
    
    let white_height: u32 = 100;
    let black_height: u32 = 70;
    let white_width: u32 = 45;
    let black_width: u32 = 30;

    let children: Vec<Box<dyn ComponentTraits>> = Vec::new();
    let mut bar: BarContainer = BarContainer::new(
        None,
        true,
        375,
        550,
        0,
        0,
        Some(children),
        Orientation::HORIZONTAL,
    );

    let box_style_1: Style = Style::new(
        Color::from_hex(0xd0d0d0),
    );

    let mut box_1: BoxComponent = BoxComponent::new(
        0,
        0,
        white_height,
        white_width,
        box_style_1,
        true,
    );


    let box_style_2: Style = Style::new(
        Color::from_hex(0x000000),
    );

    let mut box_2: BoxComponent = BoxComponent::new(
        0,
        0,
        black_height,
        black_width,
        box_style_2,
        true,
    );


    let mut box_style_3: Style = Style::new(
        Color::from_hex(0xd0d0d0),
    );
    
    let mut box_3: BoxComponent = BoxComponent::new(
        0,
        25,
        white_height,
        white_width,
        box_style_3,
        true,
    );


    let mut box_style_4: Style = Style::new(
        Color::from_hex(0x000000),
    );
    
    let mut box_4: BoxComponent = BoxComponent::new(
        0,
        0,
        black_height,
        black_width,
        box_style_4,
        true,
    );


    let mut box_style_5: Style = Style::new(
        Color::from_hex(0xd0d0d0),
    );
    
    let mut box_5: BoxComponent = BoxComponent::new(
        0,
        0,
        white_height,
        white_width,
        box_style_5,
        true,
    );

    let box_style_6: Style = Style::new(
        Color::from_hex(0xd0d0d0),
    );

    let mut box_6: BoxComponent = BoxComponent::new(
        0,
        0,
        white_height,
        white_width,
        box_style_6,
        true,
    );

    let box_style_7: Style = Style::new(
        Color::from_hex(0x000000),
    );

    let mut box_7: BoxComponent = BoxComponent::new(
        0,
        0,
        black_height,
        black_width,
        box_style_7,
        true,
    );

    let box_style_8: Style = Style::new(
        Color::from_hex(0xd0d0d0),
    );

    let mut box_8: BoxComponent = BoxComponent::new(
        0,
        0,
        white_height,
        white_width,
        box_style_8,
        true,
    );

    
    let box_style_9: Style = Style::new(
        Color::from_hex(0x000000),
    );

    let mut box_9: BoxComponent = BoxComponent::new(
        0,
        0,
        black_height,
        black_width,
        box_style_9,
        true,
    );


    let mut box_style_10: Style = Style::new(
        Color::from_hex(0xd0d0d0),
    );
    
    let mut box_10: BoxComponent = BoxComponent::new(
        0,
        0,
        white_height,
        white_width,
        box_style_10,
        true,
    );


    let mut box_style_11: Style = Style::new(
        Color::from_hex(0x000000),
    );
    
    let mut box_11: BoxComponent = BoxComponent::new(
        0,
        0,
        black_height,
        black_width,
        box_style_11,
        true,
    );


    let mut box_style_12: Style = Style::new(
        Color::from_hex(0xd0d0d0),
        Color::set_alpha(0x00),
    );
    
    let mut box_12: BoxComponent = BoxComponent::new(
        0,
        0,
        white_height,
        white_width,
        box_style_12,
        true,
    );
    

    let box_c = MyBox::new(Rc::new(RefCell::new(box_1)), Note::C);
    let box_csharp = MyBox::new(Rc::new(RefCell::new(box_2)), Note::Csharp);
    let box_d = MyBox::new(Rc::new(RefCell::new(box_3)), Note::D);
    let box_eflat = MyBox::new(Rc::new(RefCell::new(box_4)), Note::Eflat);
    let box_e = MyBox::new(Rc::new(RefCell::new(box_5)), Note::E);
    let box_f = MyBox::new(Rc::new(RefCell::new(box_6)), Note::F);
    let box_fsharp = MyBox::new(Rc::new(RefCell::new(box_7)), Note::Fsharp);
    let box_g = MyBox::new(Rc::new(RefCell::new(box_8)), Note::G);
    let box_aflat = MyBox::new(Rc::new(RefCell::new(box_9)), Note::Aflat);
    let box_a = MyBox::new(Rc::new(RefCell::new(box_10)), Note::A);
    let box_bflat = MyBox::new(Rc::new(RefCell::new(box_11)), Note::Bflat);
    let box_b = MyBox::new(Rc::new(RefCell::new(box_12)), Note::B);

    bar.add_to_children(Box::new(box_c));
    bar.add_to_children(Box::new(box_csharp));
    bar.add_to_children(Box::new(box_d));
    bar.add_to_children(Box::new(box_eflat));
    bar.add_to_children(Box::new(box_e));
    bar.add_to_children(Box::new(box_f));
    bar.add_to_children(Box::new(box_fsharp));
    bar.add_to_children(Box::new(box_g));
    bar.add_to_children(Box::new(box_aflat));
    bar.add_to_children(Box::new(box_a));
    bar.add_to_children(Box::new(box_bflat));
    bar.add_to_children(Box::new(box_b));

    run_app(bar);
}