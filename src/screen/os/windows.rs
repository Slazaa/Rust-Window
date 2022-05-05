use winapi::um::winuser::{
	SM_CXSCREEN,
	SM_CYSCREEN,
	GetSystemMetrics
};

use crate::utils::{Screen, Size};

pub fn get_size(flag: Screen) -> Size {
	unsafe {
		match flag {
			Screen::Main => return Size::new(GetSystemMetrics(SM_CXSCREEN) as u32, GetSystemMetrics(SM_CYSCREEN) as u32)
		}
	}
}