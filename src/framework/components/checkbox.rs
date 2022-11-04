use skia_safe::{ Canvas, Rect, Color4f, Paint };
use super::Style;
use super::*;
use skia_safe::font_style::Slant;
use skia_safe::font_style::Weight;
use skia_safe::font_style::Width;
use skia_safe::TextBlob;
use skia_safe::Typeface;
use skia_safe::Point;

pub struct CheckBox {
    pub id: Uuid,
    pub left: u32,
    pub top: u32,
    pub height: u32,
    pub width: u32,
    pub visible: bool,
    pub componentType: Type,
    pub checkType: CheckType,
    pub checked: bool,
    pub text: String,
    pub altText: Option<String>,
}

pub enum CheckType {
    BOX,
    RADIO,
}

impl CheckBox {
    pub fn new(
        left: u32,
        top: u32,
        height: u32,
        width: u32,
        visible: bool,
        checkType: CheckType,
        checked: bool,
        text: String,
        altText: Option<String>,
    ) -> CheckBox {
        CheckBox {
            id: Uuid::new_v4(),
            left,
            top,
            height,
            width,
            visible,
            componentType: Type::CHECKBOX,
            checkType,
            checked,
            text,
            altText,
        }
    }
}

impl Draw for CheckBox {
    fn draw(&mut self, canvas: &mut Canvas) {
        if self.visible {
            canvas.save();
            use skia_safe::{Font, FontStyle};
            //make box
            let right = self.left + self.width / 3;
            let bottom = self.top + self.height;
            let rect = Rect::new(
                self.left as f32,
                self.top as f32,
                right as f32,
                bottom as f32,
            );
            let mut col: Color4f = Color4f::new(100.0, 0.0, 0.0, 1.0);
            if self.checked {
                col = Color4f::new(80.0, 80.0, 80.0, 1.0)
            };
            let mut paint: Paint = Paint::new(
                col,
                None
            );

            //make text
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

            let mut t = self.text.as_str();
            if let None = self.altText {

            }
            else {
                if self.checked {
                    t = self.altText.as_ref().unwrap().as_str();
                }
            }
    
            let text: Option<TextBlob> = TextBlob::from_str(
                t,
                &font,
            );

            let mut p: Paint = Paint::new(
                Color4f::new(255.0, 0.0, 0.0, 1.0),
                None
            );
            p.set_style(skia_safe::PaintStyle::Fill);


            //draw
            let text_left = self.left + self.width / 3 + 20;
            let text_bot = self.top + self.height;
            canvas.draw_text_blob(
                text.unwrap(),
                Point::new(text_left as f32, text_bot as f32),
                &p,
            );
            canvas.draw_rect(rect, &paint);
            canvas.restore();
        }
    }
}

impl Find for CheckBox {
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

impl Remove for CheckBox {}

impl OnClick for CheckBox {
    fn on_click(&mut self) {
        self.checked = !self.checked;
    }
}

impl ToggleVisible for CheckBox {
    fn toggle_visible(&mut self) {
        self.visible = !self.visible;
    }
}

impl GetWidth for CheckBox {
    fn get_width(&self) -> u32 {
        return self.width;
    }
}

impl GetHeight for CheckBox {
    fn get_height(&self) -> u32 {
        return self.height;
    }
}

impl GetType for CheckBox {
    fn get_type(&self) -> Option<Type> {
        Some(Type::CHECKBOX)
    }
}

impl SetLeft for CheckBox {
    fn set_left(&mut self, _value: u32) {
        self.left = _value;
    }
}

impl SetTop for CheckBox {
    fn set_top(&mut self, _value: u32) {
        self.top = _value;
    }
}

impl SetStyle for CheckBox {}