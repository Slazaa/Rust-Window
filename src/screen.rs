mod os;

use crate::utils::{ScreenType, Size};

pub fn get_size(screen: ScreenType) -> Size {
	#[cfg(unix)]
	return os::unix::get_size(screen);

	#[cfg(windows)]
	return os::windows::get_size(screen);
}