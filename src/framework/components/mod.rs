pub struct Style {
    color: usize,
}

impl Style {
    pub fn new(color: usize) -> Style {
        Style {
            color,
        }
    }
}

pub struct BoxComponent {
    left: usize,
    top: usize,
    height: usize,
    width: usize,
    style: Style,
}

impl BoxComponent {
    pub fn new(
        left: usize,
        top: usize,
        height: usize,
        width: usize,
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
