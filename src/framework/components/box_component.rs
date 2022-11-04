use skia_safe::{ Canvas, Rect, Color4f, Paint };

use super::Style;
use super::{ Draw, GetHeight, GetWidth, SetLeft, SetTop };

pub struct BoxComponent {
    pub id: Uuid,
    pub left: u32,
    pub top: u32,
    pub height: u32,
    pub width: u32,
    pub style: Style,
    pub visible: bool,
    pub componentType: Type,
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
            id: Uuid::new_v4(),
            left,
            top,
            height,
            width,
            style,
            visible,
            componentType: Type::BOX,
        }
    }
}

impl Draw for BoxComponent {
    fn draw(&mut self, canvas: &mut Canvas) {
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

impl Find for BoxComponent {
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

impl Remove for BoxComponent {
    fn remove(&mut self, id: Uuid) -> bool {
        if id == self.id {
            true
        }
        else {
            false
        }
    }
}

impl ToggleVisible for BoxComponent {
    fn toggle_visible(&mut self) {
        self.visible = !self.visible;
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

// impl GetMutParent for BoxComponent {
//     fn get_mut_parent(&mut self) -> &mut Box<dyn ComponentTraits> {
//         return &mut self.parent;
//     }
// }

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

impl OnClick for BoxComponent {}

impl SetStyle for BoxComponent {
    fn set_style(&mut self, style: Style) {
        self.style = style;
    }
}