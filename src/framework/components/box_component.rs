use skia_safe::{ Canvas, Rect, Color4f, Paint };

use super::Style;
use super::{ Draw, GetHeight, GetWidth, SetLeft, SetTop };

pub struct BoxComponent {
    pub left: u32,
    pub top: u32,
    pub height: u32,
    pub width: u32,
    pub style: Style,
    pub visible: bool,
}

impl BoxComponent {
    pub fn new(
        left: u32,
        top: u32,
        height: u32,
        width: u32,
        style: Style,
        visible: bool,
    ) -> BoxComponent {
        BoxComponent {
            left,
            top,
            height,
            width,
            style,
            visible,
        }
    }
}

impl Draw for BoxComponent {
    fn draw(&self, canvas: &mut Canvas) {
        if self.visible {
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
