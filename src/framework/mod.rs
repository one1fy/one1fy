use crate::platform::start_event_loop;

// Expose components to everyone.
pub mod components;

use crate::components::BoxComponent;
use crate::components::BarContainer;

pub fn run_app(tree: BarContainer) {
    start_event_loop(tree);
}
