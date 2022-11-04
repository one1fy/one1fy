use crate::platform::start_event_loop;

// Expose components to everyone.
pub mod components;

use crate::components::BoxComponent;
use crate::components::BarContainer;
use crate::components::ComponentTraits;

pub fn run_app(tree: Box<dyn ComponentTraits>) {
    start_event_loop(tree);
}
