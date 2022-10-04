use skia_safe::{ Canvas, Rect, Color4f };
use skia_safe::paint::{ Paint };

pub trait Draw {
    fn draw(&self, canvas: &mut Canvas);
}

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
    pub left: u32,
    pub top: u32,
    pub height: u32,
    pub width: u32,
    pub style: Style,
}

impl BoxComponent {
    pub fn new(
        left: u32,
        top: u32,
        height: u32,
        width: u32,
        style: Style,
    ) -> BoxComponent {
        BoxComponent {
            left,
            top,
            height,
            width,
            style,
        }
    }

    pub fn set_left(&mut self, val: u32) {
        self.left = val;
    }

    pub fn set_top(&mut self, val: u32) {
        self.top = val;
    }
}

impl Draw for BoxComponent {
    fn draw(&self, canvas: &mut Canvas) {
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


