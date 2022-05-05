use std::ptr::null_mut;

use winapi::{
	shared::{
		minwindef::{
			LPARAM,
			LRESULT,
			UINT,
			WPARAM
		},
		windef::HWND,
	},
	um::{
		libloaderapi::GetModuleHandleW,
		winuser::{
			SW_SHOW,
			WS_OVERLAPPEDWINDOW,
			WNDCLASSEXW,
			CreateWindowExW,
			RegisterClassExW,
			ShowWindow,
			UpdateWindow
		}
	}
};

use crate::utils::{Position, Size, to_wstring};

pub type WindowHandle = HWND;

extern "system" fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
	todo!();
}

// TODO: - Fullscreen
pub fn create_window(size: Size, pos: Position, title: &str, fullscreen: bool, visible: bool) -> Result<WindowHandle, String> {
	unsafe {
		let class_name: Vec<u16> = to_wstring("WindowClass");
		
		let mut wc: WNDCLASSEXW = std::mem::zeroed();
		let hinstance = GetModuleHandleW(null_mut());

		wc.lpfnWndProc = Some(window_proc);
		wc.hInstance = hinstance;
		wc.lpszClassName = class_name.as_ptr();

		RegisterClassExW(&wc);

		let title = to_wstring(title).as_ptr();
		let hwnd = CreateWindowExW(
			0,
			class_name.as_ptr(),
			title, 
			WS_OVERLAPPEDWINDOW, 
			pos.x, 
			pos.y, 
			size.width as i32, 
			size.height as i32, 
			null_mut(), 
			null_mut(), 
			hinstance,
			null_mut()
		);

		if hwnd.is_null() {
			return Err("Failed creating window".to_owned());
		}

		if visible {
			ShowWindow(hwnd, SW_SHOW);
			UpdateWindow(hwnd);
		}

		Ok(hwnd)
	}
}