mod os;

use crate::utils::{Screen, Size};

pub fn get_size(screen: Screen) -> Size {
	#[cfg(unix)]
	return os::unix::get_size(screen);

	#[cfg(windows)]
	return os::windows::get_size(screen);
}