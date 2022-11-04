use skia_safe::{ Canvas, Rect, Color4f };
use skia_safe::paint::{ Paint };
use uuid::Uuid;
use std::{cell::RefCell, rc::Rc};

pub mod bar;
pub use bar::BarContainer;

pub mod checkbox;
pub use checkbox::CheckBox;

pub mod text_component;
pub use text_component::TextComponent;

pub mod box_component;
pub use box_component::BoxComponent;

pub enum Type {
    BOX,
    CONTAINER,
    TEXT,
    CHECKBOX
}


pub trait Draw {
    fn draw(&mut self, canvas: &mut Canvas);
}

//get_width
pub trait GetWidth {
    fn get_width(&self) -> u32;
}

//get_height
pub trait GetHeight {
    fn get_height(&self) -> u32;
}

//set_left
pub trait SetLeft {
    fn set_left(&mut self, value: u32);
}

//set_top
pub trait SetTop {
    fn set_top(&mut self, value: u32);
}

pub trait SetStyle {
    fn set_style(&mut self, style: Style) {

    }
}

pub trait GetType {
    fn get_type(&self) -> Option<Type> {
        println!("This component has no type");
        None
    }
}

pub trait Find {
    fn find(&mut self, x: u32, y: u32) -> Option<Uuid> {
        println!("Component has no find function defined.");
        None
    }
}

pub trait Remove {
    fn remove(&mut self, id: Uuid) -> bool {
        false
    }
}

pub trait OnClick {
    fn on_click(&mut self) {
        println!("");
    }
}

pub trait ToggleVisible {
    fn toggle_visible(&mut self) {
        print!("");
    }
}

// pub trait GetMutParent {
//     fn get_mut_parent(&mut self) -> &mut Box<dyn ComponentTraits>;
// }

pub trait ComponentTraits: Draw + GetWidth + GetHeight + SetLeft + SetTop + GetType + Find + Remove + OnClick + ToggleVisible + SetStyle {}

pub struct Style {
    pub color: Color,
}

impl Style {
    pub fn new(color: Color) -> Style {
        Style {
            color,
        }
    }
}

pub struct Color {
    pub color: u32,
}

impl Color {
    pub fn from_rgb(red: u8, blue: u8, green: u8) -> Color {
        let mut color: u32 = 0xffffffff;

        let mut r: u32 = red.into();
        r <<= 16;

        let mut b: u32 = blue.into();
        b <<= 8;

        color &= r;
        color &= b;
        color &= green as u32;

        Color {
            color,
        }
    }

    pub fn from_hex(hex: u32) -> Color {
        let mut alpha: u32 = 0xff000000;

        alpha = alpha | hex;

        Color {
            color: alpha,
        }
    }

    pub fn set_alpha(&mut self, alpha: u8) {
        let mut mask: u32 = alpha.into();
        mask <<= 24;
        self.color |= mask;
    }
}



impl<T: Draw + GetHeight + GetWidth + SetLeft + SetTop + GetType + Find + Remove + OnClick + ToggleVisible + SetStyle> ComponentTraits for T {}


