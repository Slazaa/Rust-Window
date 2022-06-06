mod os;

use crate::window::WindowHandle;
use crate::utils::Interface;

#[cfg(unix)]
pub type DeviceContextHandle = self::os::unix::DeviceContextHandle;

#[cfg(windows)]
pub type DeviceContextHandle = self::os::windows::DeviceContextHandle;

pub fn create_context(window_handle: WindowHandle, interface: Interface) -> Result<DeviceContextHandle, String> {
	#[cfg(unix)]
	return os::unix::create_context(window_handle, interface);

	#[cfg(windows)]
	return os::windows::create_context(window_handle, interface);
}

pub fn release_context(window_handle: WindowHandle, context_handle: DeviceContextHandle, interface: Interface) {
	#[cfg(unix)]
	os::unix::release_context(window_handle, context_handle, interface);

	#[cfg(windows)]
	os::windows::release_context(window_handle, context_handle, interface);
}
