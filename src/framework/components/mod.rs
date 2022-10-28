use skia_safe::{ Canvas, Rect, Color4f };
use skia_safe::paint::{ Paint };
use uuid::Uuid;

pub mod bar;
pub use bar::BarContainer;

pub enum Type {
    BOX,
    CONTAINER,
    TEXT
}

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

pub trait ComponentTraits: Draw + GetWidth + GetHeight + SetLeft + SetTop + GetType + Find {}

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

pub struct BoxComponent {
    pub id: Uuid,
    pub left: u32,
    pub top: u32,
    pub height: u32,
    pub width: u32,
    pub style: Style,
    pub visible: bool,
    pub componentType: Type,
    pub onClick: Option<fn()>
}

impl BoxComponent {
    pub fn new(
        left: u32,
        top: u32,
        height: u32,
        width: u32,
        style: Style,
        visible: bool,
        onCl: Option<fn()>
    ) -> BoxComponent {
        BoxComponent {
            id: Uuid::new_v4(),
            left,
            top,
            height,
            width,
            style,
            visible,
            componentType: Type::BOX,
            onClick: onCl,
        }
    }
}

impl Draw for BoxComponent {
    fn draw(&self, canvas: &mut Canvas) {
        if (self.visible) {
            canvas.save();
            let right = self.left + self.width;
            let bottom = self.top + self.height;
            let rect = Rect::new(
                self.left as f32,
                self.top as f32,
                right as f32,
                bottom as f32,
            );
            let mut paint: Paint = Paint::new(
                Color4f::new(0.0, 0.0, 0.0, 0.0),
                None
            );
            paint.set_color(self.style.color.color);
            canvas.draw_rect(rect, &paint);
            canvas.restore();
        }
        
    }
}

impl Find for BoxComponent {
    fn find(&mut self, x: u32, y: u32) -> Option<Uuid> {
        let right = self.left + self.width;
        let bottom = self.top + self.height;
        if x >= self.left && x <= right && y >= self.top && y <= bottom && self.visible {
            if self.onClick.is_some() {
                let f = self.onClick.unwrap();
                f();
            }
            return Some(self.id);
        }
        else {
            return None;
        }
    }
}

impl GetHeight for BoxComponent {
    fn get_height(&self) -> u32 {
        self.height
    }
}

impl GetWidth for BoxComponent {
    fn get_width(&self) -> u32 {
        self.width
    }
}

impl SetLeft for BoxComponent {
    fn set_left(&mut self, val: u32) {
        self.left = val;
    }
}

impl SetTop for BoxComponent {
    fn set_top(&mut self, val: u32) {
        self.top = val;
    }
}

impl GetType for BoxComponent {
    fn get_type(&self) -> Option<Type> {
        Some(Type::BOX)
    }
}

impl<T: Draw + GetHeight + GetWidth + SetLeft + SetTop + GetType + Find> ComponentTraits for T {}


