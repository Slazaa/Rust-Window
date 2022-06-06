mod os;

#[cfg(unix)]
pub use os::unix::*;

#[cfg(windows)]
pub use os::windows::*;

#[derive(Clone, Copy, Debug)]
pub enum ScreenType {
	Main
}
