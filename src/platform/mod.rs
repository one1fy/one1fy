#[cfg(feature = "windows")]
pub mod windows;

#[cfg(feature = "windows")]
pub use windows::start_event_loop;

#[cfg(feature = "macos")]
pub mod macos;

#[cfg(feature = "macos")]
pub use macos::start_event_loop_mac;


/*
#[cfg(not(feature = "windows"))]
pub fn run_platform() {
    print!("Unimplemented");
}
*/