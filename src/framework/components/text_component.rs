use skia_safe::TextBlob;
use skia_safe::Canvas;

pub struct TextComponent {
    pub left: u32,
    pub top: u32,
    pub font_size: u32,
    pub text: String,
}

impl TextComponent {
    pub fn new(
        left: u32,
        top: u32,
        font_size: u32,
        text: String,
    ) -> TextComponent {
        TextComponent {
            left,
            top,
            font_size,
            text,
        }
    }
}

impl Draw for TextComponent {
    fn draw(&self, canvas: &mut Canvas) {
        canvas.save();

        use skia_safe::{Font, FontStyle, Weight};

        let font: Font = Font::new(
            Typeface::new(
                "Arial",
                FontStyle::new(
                    Weight::NORMAL,
                    Width::NORMAL,
                    Slant::NORMAL,
                ),
            ),
            Scalar::ONE,
        );

        TextBlob::from_str(
            self.text.as_str(),

        );
    }
}