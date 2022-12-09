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
    pub title: Option<String>,
}

impl MyBox {
    pub fn new(
        child: Rc<RefCell<dyn ComponentTraits>>,
        tone: f32,
        title: Option<String>
    ) -> MyBox {
        MyBox {
            child,
            tone,
            title,
        }
    }

    pub fn play_tone(&mut self, note: f32) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Add a dummy source of the sake of the example.
        let source_a = SineWave::new(note).take_duration(Duration::from_secs_f32(0.5)).amplify(0.20);
        sink.append(source_a);

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
        //println!("searching...");
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

impl SetText for MyBox {}

impl OnClick for MyBox {
    fn on_click(&mut self) {
        let num = rand::thread_rng().gen_range(0..0xffffffff);
        let style: Style = Style::new(Color::from_hex(num));
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
        pub const C_SHARP: f32 = 554.37;
        pub const D: f32 = 587.33;
        pub const E_FLAT: f32 = 622.25;
        pub const E: f32 = 659.25;
        pub const F: f32 = 698.46;
        pub const F_SHARP: f32 = 739.99;
        pub const G: f32 = 783.99;
        pub const A_FLAT: f32 = 830.61;
        pub const A: f32 = 880.00;
        pub const B_FLAT: f32 = 932.33;
        pub const B: f32 = 987.77;
        pub const C_OCTAVE: f32 = 1046.50;
    }


#[cfg(any(feature = "windows", feature = "macos"))]
fn main() {
    build();
}


fn build() {
    
    let white_height: u32 = 100;
    let black_height: u32 = 60;
    let white_width: u32 = 40;
    let black_width: u32 = 25;
    let top_position: u32 = 100;

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
        true,
    );

    let box_style_1: Style = Style::new(
        Color::from_hex(0xfffff0),
    );

    let mut box_1: BoxComponent = BoxComponent::new(
        24,
        top_position,
        white_height,
        white_width,
        box_style_1,
        true,
        " ".to_string(),
    );


    let box_style_2: Style = Style::new(
        Color::from_hex(0x000000),
    );

    let mut box_2: BoxComponent = BoxComponent::new(
        52,
        top_position,
        black_height,
        black_width,
        box_style_2,
        true,
        " ".to_string(),
    );


    let mut box_style_3: Style = Style::new(
        Color::from_hex(0xfffff0),
    );
    
    let mut box_3: BoxComponent = BoxComponent::new(
        65,
        top_position,
        white_height,
        white_width,
        box_style_3,
        true,
        " ".to_string(),
    );


    let mut box_style_4: Style = Style::new(
        Color::from_hex(0x000000),
    );
    
    let mut box_4: BoxComponent = BoxComponent::new(
        93,
        top_position,
        black_height,
        black_width,
        box_style_4,
        true,
        " ".to_string(),
    );


    let mut box_style_5: Style = Style::new(
        Color::from_hex(0xfffff0),
    );
    
    let mut box_5: BoxComponent = BoxComponent::new(
        106,
        top_position,
        white_height,
        white_width,
        box_style_5,
        true,
        " ".to_string(),
    );

    let box_style_6: Style = Style::new(
        Color::from_hex(0xfffff0),
    );

    let mut box_6: BoxComponent = BoxComponent::new(
        147,
        top_position,
        white_height,
        white_width,
        box_style_6,
        true,
        " ".to_string(),
    );

    let box_style_7: Style = Style::new(
        Color::from_hex(0x000000),
    );

    let mut box_7: BoxComponent = BoxComponent::new(
        175,
        top_position,
        black_height,
        black_width,
        box_style_7,
        true,
        " ".to_string(),
    );

    let box_style_8: Style = Style::new(
        Color::from_hex(0xfffff0),
    );

    let mut box_8: BoxComponent = BoxComponent::new(
        188,
        top_position,
        white_height,
        white_width,
        box_style_8,
        true,
        " ".to_string(),
    );

    
    let box_style_9: Style = Style::new(
        Color::from_hex(0x000000),
    );

    let mut box_9: BoxComponent = BoxComponent::new(
        216,
        top_position,
        black_height,
        black_width,
        box_style_9,
        true,
        " ".to_string(),
    );


    let mut box_style_10: Style = Style::new(
        Color::from_hex(0xfffff0),
    );
    
    let mut box_10: BoxComponent = BoxComponent::new(
        229,
        top_position,
        white_height,
        white_width,
        box_style_10,
        true,
        " ".to_string(),
    );


    let mut box_style_11: Style = Style::new(
        Color::from_hex(0x000000),
    );
    
    let mut box_11: BoxComponent = BoxComponent::new(
        257,
        top_position,
        black_height,
        black_width,
        box_style_11,
        true,
        " ".to_string(),
    );


    let mut box_style_12: Style = Style::new(
        Color::from_hex(0xfffff0),
    );
    
    let mut box_12: BoxComponent = BoxComponent::new(
        270,
        top_position,
        white_height,
        white_width,
        box_style_12,
        true,
        " ".to_string(),
    );

    let mut box_style_13: Style = Style::new(
        Color::from_hex(0xfffff0),
    );
    
    let mut box_13: BoxComponent = BoxComponent::new(
        311,
        top_position,
        white_height,
        white_width,
        box_style_13,
        true,
        " ".to_string(),
    );

    let mut c = Color::from_hex(0xff00ff);
    c.set_alpha(0x00);

    let mut box_style_text: Style = Style::new(
        //Color::from_hex(0xFF00FF),
        c,
    );
    
    let mut box_text: BoxComponent = BoxComponent::new(
        20,
        0,
        50,
        200,
        box_style_text,
        true,
        "My Piano App".to_string(),
    );
    

    let box_c = MyBox::new(Rc::new(RefCell::new(box_1)), Note::C, None);
    let box_c_sharp = MyBox::new(Rc::new(RefCell::new(box_2)), Note::C_SHARP, None);
    let box_d = MyBox::new(Rc::new(RefCell::new(box_3)), Note::D, None);
    let box_e_flat = MyBox::new(Rc::new(RefCell::new(box_4)), Note::E_FLAT, None);
    let box_e = MyBox::new(Rc::new(RefCell::new(box_5)), Note::E, None);
    let box_f = MyBox::new(Rc::new(RefCell::new(box_6)), Note::F, None);
    let box_f_sharp = MyBox::new(Rc::new(RefCell::new(box_7)), Note::F_SHARP, None);
    let box_g = MyBox::new(Rc::new(RefCell::new(box_8)), Note::G, None);
    let box_a_flat = MyBox::new(Rc::new(RefCell::new(box_9)), Note::A_FLAT, None);
    let box_a = MyBox::new(Rc::new(RefCell::new(box_10)), Note::A, None);
    let box_b_flat = MyBox::new(Rc::new(RefCell::new(box_11)), Note::B_FLAT, None);
    let box_b = MyBox::new(Rc::new(RefCell::new(box_12)), Note::B, None);
    let box_c_octave = MyBox::new(Rc::new(RefCell::new(box_13)), Note::C_OCTAVE, None);


    let text: String = "My Piano App".to_string();
    let box_title = MyBox::new(Rc::new(RefCell::new(box_text)), 0.0, Some(text.clone()));


    //bar.add_to_children(Box::new(box_title));

    bar.add_to_children(Box::new(box_c));
    bar.add_to_children(Box::new(box_d));
    bar.add_to_children(Box::new(box_e));
    bar.add_to_children(Box::new(box_f));
    bar.add_to_children(Box::new(box_g));
    bar.add_to_children(Box::new(box_a));
    bar.add_to_children(Box::new(box_b));
    bar.add_to_children(Box::new(box_c_octave));

    bar.add_to_children(Box::new(box_c_sharp));
    bar.add_to_children(Box::new(box_e_flat));
    bar.add_to_children(Box::new(box_f_sharp));
    bar.add_to_children(Box::new(box_a_flat));
    bar.add_to_children(Box::new(box_b_flat));

    run_app(bar);
}