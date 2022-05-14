use winapi::um::winuser::{
	SM_CXSCREEN,
	SM_CYSCREEN,
	GetSystemMetrics
};

use crate::utils::Size;
use crate::screen::ScreenType;

pub fn get_size(flag: ScreenType) -> Size {
	unsafe {
		match flag {
			ScreenType::Main => return Size::new(GetSystemMetrics(SM_CXSCREEN) as u32, GetSystemMetrics(SM_CYSCREEN) as u32)
		}
	}
}