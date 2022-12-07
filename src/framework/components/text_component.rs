use skia_safe::Color4f;
use skia_safe::Paint;
use skia_safe::Point;
use skia_safe::TextBlob;
use skia_safe::Canvas;
use skia_safe::Typeface;
use skia_safe::font_style::Slant;
use skia_safe::font_style::Weight;
use skia_safe::font_style::Width;
use super::*;

pub struct TextComponent {
    pub id: Uuid,
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
    pub font_size: u32,
    pub visible: bool,
    pub text: String,
    pub color: crate::components::Color,
    pub componentType: Type,
}

impl TextComponent {
    pub fn new(
        left: u32,
        top: u32,
        width: u32,
        height: u32,
        font_size: u32,
        visible: bool,
        text: String,
        color: crate::components::Color,
    ) -> TextComponent {
        TextComponent {
            id: Uuid::new_v4(),
            left,
            top,
            width,
            height,
            font_size,
            visible,
            text,
            color,
            componentType: Type::TEXT,
        }
    }
}

impl Draw for TextComponent {
    fn draw(&mut self, canvas: &mut Canvas) {
        if self.text.len() > 0 {
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

            let mut text: Option<TextBlob> = TextBlob::from_str(
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
                Point::new(self.left as f32, self.top as f32),
                &paint,
            );
        }
        
    }
}

impl Find for TextComponent {
    fn find(&mut self, x: u32, y: u32) -> Option<Uuid> {
        let right = self.left + self.width;
        let bottom = self.top + self.height;
        if x >= self.left && x <= right && y >= self.top && y <= bottom && self.visible {
            self.on_click();
            return Some(self.id);
        }
        else {
            return None;
        }
    }
}

impl Remove for TextComponent {}

impl OnClick for TextComponent {}

impl OnPress for TextComponent {}

impl ToggleVisible for TextComponent {
    fn toggle_visible(&mut self) {
        self.visible = !self.visible;
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

impl GetType for TextComponent {
    fn get_type(&self) -> Option<Type> {
        Some(Type::TEXT)
    }
}

impl GetText for TextComponent {
    fn get_text(&mut self) -> Option<String> {
        Some(self.text.clone())
    }
}

impl SetLeft for TextComponent {
    fn set_left(&mut self, _value: u32) {
        self.left = _value;
    }
}

impl SetTop for TextComponent {
    fn set_top(&mut self, _value: u32) {
        self.top = _value;
    }
}

impl SetText for TextComponent {
    fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl SetStyle for TextComponent {}