pub struct Style {
    pub color: u32,
}

impl Style {
    pub fn new(color: u32) -> Style {
        Style {
            color,
        }
    }
}

pub struct BoxComponent {
    pub left: f32,
    pub top: f32,
    pub height: f32,
    pub width: f32,
    pub style: Style,
}

impl BoxComponent {
    pub fn new(
        left: f32,
        top: f32,
        height: f32,
        width: f32,
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
}
