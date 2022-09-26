use crate::platform::start_event_loop;

// Expose components to everyone.
pub mod components;

use crate::components::BoxComponent;

pub fn run_app(tree: BoxComponent) {
    start_event_loop(tree);
}

// pub fn run_app_mac(tree: BoxComponent) {
//     start_event_loop_mac(tree);
// }