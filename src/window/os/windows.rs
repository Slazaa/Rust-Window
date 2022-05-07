use std::ptr::{null_mut, null};

use winapi::{
	shared::{
		basetsd::LONG_PTR,
		minwindef::{
			FALSE,
			LPARAM,
			LRESULT,
			UINT,
			WPARAM
		},
		windef::{
			HBRUSH,
			HWND
		},
	},
	um::{
		libloaderapi::GetModuleHandleW,
		winuser::{
			COLOR_WINDOW,
			GWLP_USERDATA,
			IDI_APPLICATION,
			IDC_ARROW,
			PM_REMOVE,
			SW_SHOW,
			WM_CLOSE,
			WM_CREATE,
			WM_DESTROY,
			WS_OVERLAPPEDWINDOW,
			CREATESTRUCTW,
			MSG,
			WNDCLASSEXW,
			CreateWindowExW,
			DefWindowProcW,
			DestroyWindow,
			DispatchMessageW,
			GetWindowLongPtrW,
			LoadCursorW,
			LoadIconW,
			PeekMessageW,
			PostQuitMessage,
			RegisterClassExW,
			SetWindowLongPtrW,
			ShowWindow,
			TranslateMessage,
			UpdateWindow
		}
	}
};

use crate::event::Event;
use crate::utils::{Position, Size, to_wstring};

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
			WM_DESTROY => PostQuitMessage(0),
			_ => return DefWindowProcW(h_wnd, msg, w_param, l_param)
		}

		0
	}
}

// TODO: - Fullscreen
pub fn create_window(size: Size, pos: Position, title: &str, fullscreen: bool, visible: bool) -> WindowHandle {
	unsafe {
		let class_name: Vec<u16> = to_wstring("WindowClass");
		
		let mut wc: WNDCLASSEXW = std::mem::zeroed();
		let h_instance = GetModuleHandleW(null_mut());

		wc.cbSize = std::mem::size_of::<WNDCLASSEXW>() as u32;
		wc.lpfnWndProc = Some(window_proc);
		wc.hInstance = h_instance;
		wc.hIcon = LoadIconW(h_instance, IDI_APPLICATION);
		wc.hCursor = LoadCursorW(h_instance, IDC_ARROW);
		wc.hbrBackground = (COLOR_WINDOW + 1) as HBRUSH;
		wc.lpszMenuName = null();
		wc.lpszClassName = class_name.as_ptr();
		wc.hIconSm = LoadIconW(h_instance, IDI_APPLICATION);

		RegisterClassExW(&wc);

		let title = to_wstring(title).as_ptr();
		let lparam: *mut Event = Box::leak(Box::new(Event::None));
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
			h_instance,
			lparam.cast()
		);

		if visible {
			ShowWindow(hwnd, SW_SHOW);
			UpdateWindow(hwnd);
		}

		hwnd
	}
}

pub fn destroy_window(handle: WindowHandle) {
	unsafe {
		let ptr = GetWindowLongPtrW(handle, GWLP_USERDATA) as *mut i32;
		Box::from_raw(ptr);

		DestroyWindow(handle);
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