use skia_safe::Color4f;
use skia_safe::Paint;
use skia_safe::Point;
use skia_safe::TextBlob;
use skia_safe::Canvas;
use skia_safe::Typeface;
use skia_safe::font_style::Slant;
use skia_safe::font_style::Weight;
use skia_safe::font_style::Width;

use super::Draw;
use super::GetHeight;
use super::GetWidth;
use super::SetLeft;
use super::SetTop;

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
    fn draw(&self, canvas: &mut Canvas) {
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

        paint.set_style(skia_safe::PaintStyle::Fill);
        paint.set_color(self.color.color);

        canvas.draw_text_blob(
            text.unwrap(),
            Point::new(100.0, 100.0),
            &paint,
        );
    }
}

impl GetWidth for TextComponent {
    fn get_width(&self) -> u32 {
        return 0;
    }
}

impl GetHeight for TextComponent {
    fn get_height(&self) -> u32 {
        return 0;
    }
}

impl SetLeft for TextComponent {
    fn set_left(&mut self, _value: u32) {
    }
}

impl SetTop for TextComponent {
    fn set_top(&mut self, _value: u32) {
    }
}