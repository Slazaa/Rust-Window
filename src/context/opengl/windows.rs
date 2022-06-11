use winapi::{
	um::{
		wingdi::{
			PIXELFORMATDESCRIPTOR,
			PFD_DOUBLEBUFFER,
			PFD_DRAW_TO_WINDOW,
			PFD_MAIN_PLANE,
			PFD_SUPPORT_OPENGL,
			PFD_TYPE_RGBA,
			ChoosePixelFormat,
			SetPixelFormat
		},
		winuser::{
			GetDC,
			ReleaseDC
		}
	},
	shared::minwindef::FALSE
};

use crate::window::WindowHandle;
use crate::context::windows::DeviceContextHandle;

pub fn create_context(window_handle: WindowHandle) -> Result<DeviceContextHandle, String> {
	unsafe {
        let mut pfd: PIXELFORMATDESCRIPTOR = std::mem::zeroed();

        pfd.nSize = std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16;
        pfd.nVersion = 1;
        pfd.dwFlags = PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER;
        pfd.iPixelType = PFD_TYPE_RGBA;
        pfd.cColorBits = 32;
        //pfd.cRedBits = 0;
        //pfd.cRedShift = 0;
        //pfd.cGreenBits = 0;
        //pfd.cGreenShift = 0;
        //pfd.cBlueBits = 0;
        //pfd.cBlueShift = 0;
        //pfd.cAlphaBits = 0;
        //pfd.cAlphaShift = 0;
        //pfd.cAccumBits = 0;
        //pfd.cAccumRedBits = 0;
        //pfd.cAccumGreenBits = 0;
        //pfd.cAccumBlueBits = 0;
        //pfd.cAccumAlphaBits = 0;
        pfd.cDepthBits = 24;
        pfd.cStencilBits = 8;
        //pfd.cAuxBuffers = 0;
        pfd.iLayerType = PFD_MAIN_PLANE;
        //pfd.bReserved = 0;
        //pfd.dwLayerMask = 0;
        //pfd.dwVisibleMask = 0;
        //pfd.dwDamageMask = 0;

        let dc = GetDC(window_handle);

        if dc.is_null() {
            return Err("Failed to get device context".to_owned());
        }

        let pixel_format = ChoosePixelFormat(dc, &pfd);

        if pixel_format == 0 {
            return Err("Failed to choose pixel format".to_owned());
        }

        if SetPixelFormat(dc, pixel_format, &pfd) == FALSE {
            return Err("Failed to set pixel format".to_owned());
        }

        Ok(dc)
	}
}

pub fn release_context(window_handle: WindowHandle, context_handle: DeviceContextHandle) {
	unsafe {
        ReleaseDC(window_handle, context_handle);
	}
}
