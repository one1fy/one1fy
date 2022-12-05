use glutin::event::MouseButton;
use glutin::event::ElementState;
use glutin::dpi::PhysicalPosition;

pub fn handle_click(position: PhysicalPosition<f64>, state: ElementState, button: MouseButton) {
    println!("The position is: {:?}", position);
    println!("The state is: {:?}", state);
    println!("The button is: {:?}\n", button);
}