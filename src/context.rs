mod os;

use crate::window::WindowHandle;

#[cfg(unix)]
pub type DeviceContextHandle = self::os::unix::DeviceContextHandle;

#[cfg(windows)]
pub type DeviceContextHandle = self::os::windows::DeviceContextHandle;

pub fn create_context(window_handle: WindowHandle) -> DeviceContextHandle {
	#[cfg(unix)]
	return os::unix::create_context(window_handle);

	#[cfg(windows)]
	return os::windows::create_context(window_handle);
}

pub fn release_context(window_handle: WindowHandle, context_handle: DeviceContextHandle) {
	#[cfg(unix)]
	os::unix::release_context(window_handle, context_handle);

	#[cfg(windows)]
	os::windows::release_context(window_handle, context_handle);
}