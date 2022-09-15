use glutin::event::MouseButton;
use glutin::event::ElementState;
use glutin::dpi::PhysicalPosition;

pub fn handle_click(position: PhysicalPosition<f64>, state: ElementState, button: MouseButton) {
    println!("click");
}