use skia_safe::Canvas;

pub mod bar;
pub use bar::BarContainer;

pub mod text_component;
pub use text_component::TextComponent;

pub mod box_component;
pub use box_component::BoxComponent;

pub trait Draw {
    fn draw(&self, canvas: &mut Canvas);
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

pub trait OpenLink {
    fn open_link(&mut self, link: String) {

    }
}

pub trait ComponentTraits: Draw + GetWidth + GetHeight + SetLeft + SetTop + OpenLink {}

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

impl<T: Draw + GetHeight + GetWidth + SetLeft + SetTop + OpenLink> ComponentTraits for T {}


