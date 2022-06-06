use std::ptr::{null_mut, null};
use std::ffi::CString;

use winapi::{
	shared::{
		basetsd::LONG_PTR,
		minwindef::*,
		windef::{
			HBRUSH,
			HWND,
			RECT
		},
	},
	um::{
		libloaderapi::GetModuleHandleW,
		winuser::*
	}
};

use crate::utils::{Position, Size, to_wstring};
use crate::event::Event;
use crate::window::Style;

pub type WindowHandle = HWND;

extern "system" fn window_proc(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
	unsafe {
		match msg {
			WM_CREATE => {
				let create_struct: *mut CREATESTRUCTW = l_param as *mut _;

				if create_struct.is_null() {
					return 0;
				}

				let boxed_i32_ptr = (*create_struct).lpCreateParams;
				SetWindowLongPtrW(h_wnd, GWLP_USERDATA, boxed_i32_ptr as LONG_PTR);

				return 1;
			}
			WM_CLOSE => {
				let ptr = GetWindowLongPtrW(h_wnd, GWLP_USERDATA) as *mut Event;
				*ptr = Event::Close;
			}
			WM_KEYDOWN => {
				let ptr = GetWindowLongPtrW(h_wnd, GWLP_USERDATA) as *mut Event;
				*ptr = Event::KeyDown(char::from_u32(w_param as u32).unwrap());
			},
			WM_KEYUP => {
				let ptr = GetWindowLongPtrW(h_wnd, GWLP_USERDATA) as *mut Event;
				*ptr = Event::KeyUp(char::from_u32(w_param as u32).unwrap());
			},
			WM_DESTROY => PostQuitMessage(0),
			_ => {
				let ptr = GetWindowLongPtrW(h_wnd, GWLP_USERDATA) as *mut Event;
				
				if !ptr.is_null() {
					*ptr = Event::None;
				}

				return DefWindowProcW(h_wnd, msg, w_param, l_param);
			}
		}

		0
	}
}

pub fn create_window(size: Size, pos: Position, title: &str, style: Style) -> Result<WindowHandle, String> {
	unsafe {
		let class_name = CString::new("WindowClass".as_bytes()).unwrap();
		
		let mut wc: WNDCLASSEXW = std::mem::zeroed();
		let h_instance = GetModuleHandleW(null_mut());

		if h_instance.is_null() {
			return Err("Failed to get module handle".to_owned());
		}

		wc.cbSize = std::mem::size_of::<WNDCLASSEXW>() as u32;
		//wc.style = 0;
		wc.lpfnWndProc = Some(window_proc);
		//wc.cbClsExtra = 0;
		//wc.cbWndExtra = 0;
		wc.hInstance = h_instance;
		wc.hIcon = LoadIconW(null_mut(), IDI_APPLICATION);
		wc.hCursor = LoadCursorW(null_mut(), IDC_ARROW);
		wc.hbrBackground = (COLOR_WINDOW + 1) as HBRUSH;
		wc.lpszMenuName = null();
		wc.lpszClassName = class_name.as_ptr() as *const u16;
		wc.hIconSm = LoadIconW(h_instance, IDI_APPLICATION);

		if RegisterClassExW(&wc) == 0 {
			return Err("Failed to register window class".to_owned());
		}

		let mut dw_style = 0;

		if style.visible {
			dw_style |= WS_VISIBLE;
		}

		if style.border {
			dw_style |= WS_BORDER;
		}

		if style.titlebar {
			dw_style |= WS_CAPTION;
		}

		if style.close {
			dw_style |= WS_SYSMENU;
		}

		if style.maximize {
			dw_style |= WS_MAXIMIZEBOX;
		}

		if style.minimize {
			dw_style |= WS_MINIMIZEBOX;
		}

		if style.resize {
			dw_style |= WS_SIZEBOX;
		}

		let title = to_wstring(title);
		let lparam: *mut Event = Box::leak(Box::new(Event::None));
		let h_wnd = CreateWindowExW(
			0,
			class_name.as_ptr() as *const u16,
			title.as_ptr() as *const u16, 
			dw_style, 
			pos.x, 
			pos.y, 
			size.width as i32, 
			size.height as i32, 
			null_mut(), 
			null_mut(),
			h_instance,
			lparam.cast()
		);

		if h_wnd.is_null() {
			return Err("Failed to create a window".to_owned()); 
		}

		UpdateWindow(h_wnd);

		if style.fullscreen {
			set_fullsreen(h_wnd, true);
		}

		Ok(h_wnd)
	}
}

pub fn destroy_window(handle: WindowHandle) {
	unsafe {
		let ptr = GetWindowLongPtrW(handle, GWLP_USERDATA) as *mut i32;
		Box::from_raw(ptr);

		DestroyWindow(handle);
	}
}

pub fn get_position(handle: WindowHandle) -> Position {
	unsafe {
		let mut rect: RECT = std::mem::zeroed();
		GetWindowRect(handle, &mut rect);

		Position::new(rect.left, rect.top)
	}
}

pub fn get_size(handle: WindowHandle) -> Size {
	unsafe {
		let mut rect: RECT = std::mem::zeroed();
		GetWindowRect(handle, &mut rect);

		Size::new((rect.right - rect.left) as u32, (rect.bottom - rect.top) as u32)
	}
}

pub fn set_position(handle: WindowHandle, pos: Position) {
	unsafe {
		SetWindowPos(handle, null_mut(), pos.x, pos.y, 0, 0, SWP_NOZORDER | SWP_NOSIZE);
	}
}

pub fn set_size(handle: WindowHandle, size: Size) {
	unsafe {
		SetWindowPos(handle, null_mut(), 0, 0, size.width as i32, size.height as i32, SWP_NOZORDER | SWP_NOMOVE | SWP_SHOWWINDOW);
	}
}

pub fn set_fullsreen(handle: WindowHandle, toggle: bool) {
	unsafe {
		todo!();
	}
}

pub fn poll_event(handle: WindowHandle, event: &mut Event) -> bool {
	unsafe {
		let mut msg: MSG = std::mem::zeroed();

		if PeekMessageW(&mut msg, handle, 0, 0, PM_REMOVE) == FALSE {
			return false;
		}

		TranslateMessage(&msg);
		DispatchMessageW(&msg);

		let ptr = GetWindowLongPtrW(handle, GWLP_USERDATA) as *mut Event;
		*event = *ptr;

		true
	}
}
