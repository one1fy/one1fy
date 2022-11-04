use skia_safe::{ Color, Canvas };

use crate::components::*;

/*
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
*/

pub fn handle_redraw(canvas: &mut Canvas, tree: &mut Box<dyn ComponentTraits>) {
    canvas.clear(Color::WHITE);
    tree.draw(canvas);
}