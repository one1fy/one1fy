use skia_safe::Color4f;
use skia_safe::Paint;
use skia_safe::Point;
use skia_safe::Scalar;
use skia_safe::TextBlob;
use skia_safe::Canvas;
use skia_safe::Typeface;
use skia_safe::font_style::Slant;
use skia_safe::font_style::Weight;
use skia_safe::font_style::Width;
use skia_safe::wrapper::ValueWrapper;

use super::Component_Traits;
use super::Draw;
use super::Get_Height;
use super::Get_Width;
use super::Set_Left;
use super::Set_Top;

pub struct TextComponent {
    pub left: u32,
    pub top: u32,
    pub font_size: u32,
    pub text: String,
    pub color: crate::components::Color,
}

impl TextComponent {
    pub fn new(
        left: u32,
        top: u32,
        font_size: u32,
        text: String,
        color: crate::components::Color,
    ) -> TextComponent {
        TextComponent {
            left,
            top,
            font_size,
            text,
            color,
        }
    }
}

impl Draw for TextComponent {
    fn draw(&mut self, canvas: &mut Canvas) {
        canvas.save();

        use skia_safe::{Font, FontStyle};

        let typeface: Option<Typeface> = Typeface::new(
            "Times New Roman",
            FontStyle::new(
                Weight::NORMAL,
                Width::NORMAL,
                Slant::Upright,
            ),
        );

        let font: Font = Font::new(
            typeface.unwrap(),
            Some(64.0 as f32),
        );

        let text: Option<TextBlob> = TextBlob::from_str(
            self.text.as_str(),
            &font,
        );

        let mut paint: Paint = Paint::new(
            Color4f::new(255.0, 0.0, 0.0, 0.0),
            None
        );

        use crate::components::Color;
        //let color: Color = Color::from_hex(0xFF0000);

        paint.set_style(skia_safe::PaintStyle::Fill);
        paint.set_color(self.color.color);

        canvas.draw_text_blob(
            text.unwrap(),
            Point::new(100.0, 100.0),
            &paint,
        );

        let alpha: u32 = 0xFF000000;
        self.color.color += 0x0F;
        if self.color.color == 0xFFFFFFFF {
            self.color.color = 0xFF000000;
        }
        self.color.color |= alpha;
        // println!("color: {:#08X}", self.color.color);
    }
}

impl Get_Width for TextComponent {
    fn get_width(&self) -> u32 {
        return 0;
    }
}

impl Get_Height for TextComponent {
    fn get_height(&self) -> u32 {
        return 0;
    }
}

impl Set_Left for TextComponent {
    fn set_left(&mut self, value: u32) {
    }
}

impl Set_Top for TextComponent {
    fn set_top(&mut self, value: u32) {
    }
}