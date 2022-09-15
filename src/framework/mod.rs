use crate::platform::start_event_loop;

// Expose components to everyone.
pub mod components;

use crate::components::BoxComponent;

pub fn run_app(tree: BoxComponent) {
    start_event_loop(tree);
}