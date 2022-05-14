mod os;

use crate::window::WindowHandle;

#[cfg(unix)]
pub type ContextHandle = self::os::unix::ContextHandle;

#[cfg(windows)]
pub type ContextHandle = self::os::windows::ContextHandle;

pub fn create_context(handle: WindowHandle) -> ContextHandle {
	#[cfg(unix)]
	return os::unix::create_context(handle);

	#[cfg(windows)]
	return os::windows::create_context(handle);
}