use skia_safe::{ Color, Canvas, Rect, Color4f };
use skia_safe::paint::{ Paint };

use crate::components::*;
// use crate::components::Draw;
// use crate::components::BarContainer;

fn draw_square(
    canvas: &mut Canvas,
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
    color: u32,
) {
    canvas.save();

    let rect = Rect::new(
        left as f32,
        top as f32,
        right as f32,
        bottom as f32,
    );

    let mut paint: Paint = Paint::new(
        Color4f::new(0.0, 0.0, 0.0, 0.0),
        None
    );

    paint.set_color(color);
    canvas.draw_rect(rect, &paint);
    canvas.restore();
}

pub fn handle_redraw(canvas: &mut Canvas, tree: &mut BarContainer) {
    canvas.clear(Color::WHITE);
    tree.draw(canvas);
}