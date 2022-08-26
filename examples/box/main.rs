use one1fy_platform::run_platform;
use skia_safe::{ Color, Canvas, Rect, Color4f };
use skia_safe::paint::{ Paint };
use glutin::event::{ ElementState, MouseButton };
use glutin::dpi::PhysicalPosition;

#[cfg(feature = "windows")]
fn main() {
    fn draw_square(
        canvas: &mut Canvas,
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    ) {
        canvas.save();
        let rect = Rect::new(left, top, right, bottom);
        let mut paint: Paint = Paint::new(
            Color4f::new(0.0, 0.0, 0.0, 0.0),
            None
        );
        paint.set_color(Color::BLUE);
        canvas.draw_rect(rect, &paint);
        canvas.restore();
    }

    fn handle_redraw(canvas: &mut Canvas) {
        static mut X: f32 = 0.0;
        unsafe {
            canvas.clear(Color::WHITE);
            draw_square(canvas, X, 0.0, 100.0 + X, 100.0);
            X += 0.5;

            if X > 375.0 {
                X = 0.0;
            }
        }
    }

    fn handle_click(position: PhysicalPosition<f64>, state: ElementState, button: MouseButton) {
        println!("click");
    }

    run_platform(handle_redraw, handle_click);
}
