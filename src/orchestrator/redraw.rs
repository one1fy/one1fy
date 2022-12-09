use skia_safe::{ Color, Canvas };

use crate::components::*;


pub fn handle_redraw(canvas: &mut Canvas, tree: &mut BarContainer) {
    canvas.clear(Color::DARK_GRAY);
    tree.draw(canvas);
}