use glutin::event::MouseButton;
use glutin::event::ElementState;
use glutin::dpi::PhysicalPosition;
use crate::framework::components::BarContainer;
use crate::framework::components::*;

pub fn handle_click(position: PhysicalPosition<f64>, state: ElementState, button: MouseButton, tree: &mut BarContainer) {
    // println!("{}, {}", position.x as u32, position.y as u32);
    let comp = tree.find(position.x as u32, position.y as u32);
    if let None = comp {
        println!("no component found");
    }
    else {
        println!("Found component: {}", comp.unwrap());
    }
}