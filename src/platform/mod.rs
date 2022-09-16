#[cfg(feature = "windows")]
pub mod windows;

#[cfg(feature = "windows")]
pub use windows::start_event_loop;


/*
#[cfg(not(feature = "windows"))]
pub fn run_platform() {
    print!("Unimplemented");
}
*/