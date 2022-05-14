mod os;

use crate::window::WindowHandle;

#[cfg(unix)]
pub type ContextHandle = self::os::unix::ContextHandle;

#[cfg(windows)]
pub type ContextHandle = self::os::windows::ContextHandle;

pub fn create_context(window_handle: WindowHandle) -> ContextHandle {
	#[cfg(unix)]
	return os::unix::create_context(window_handle);

	#[cfg(windows)]
	return os::windows::create_context(window_handle);
}

pub fn release_context(window_handle: WindowHandle, context_handle: ContextHandle) {
	#[cfg(unix)]
	os::unix::release_context(window_handle, context_handle);

	#[cfg(windows)]
	os::windows::release_context(window_handle, context_handle);
}