pub mod opengl;
pub mod unix;
pub mod windows;

#[cfg(unix)]
pub use unix::DeviceContextHandle;

#[cfg(windows)]
pub use windows::DeviceContextHandle;