#[cfg(feature = "windows")]
pub mod windows;

#[cfg(feature = "windows")]
pub use windows::start_event_loop;

#[cfg(feature = "macos")]
pub mod macos;

#[cfg(feature = "macos")]
pub use macos::start_event_loop;

#[cfg(not(any(feature = "windows", feature = "macos")))]
pub fn start_event_loop(tree: &dyn Component_Traits) {
    println!("You need to specify your target OS");
}