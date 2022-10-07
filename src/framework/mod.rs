use crate::platform::start_event_loop;

// Expose components to everyone.
pub mod components;

use crate::components::BoxComponent;
use crate::components::BarContainer;

use self::components::Component_Traits;

pub fn run_app(tree: Box<dyn Component_Traits>) {
    start_event_loop(tree);
}
