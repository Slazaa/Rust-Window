pub mod unix;
pub mod windows;

#[cfg(unix)]
pub use unix::*;

#[cfg(windows)]
pub use windows::*;

#[derive(Clone, Copy, Debug)]
pub enum ScreenType {
	Main
}
