mod os;

use crate::event::Event;
use crate::utils::{ScreenType, Position, Size};
use crate::screen;

#[cfg(unix)]
pub type WindowHandle = self::os::unix::WindowHandle;

#[cfg(windows)]
pub type WindowHandle = self::os::windows::WindowHandle;

pub struct WindowBuilder {
	size: Size,
	pos: Position,
	title: String,
	fullscreen: bool
}

impl WindowBuilder {
	pub fn size(&mut self, size: Size) -> &mut Self {
		self.size = size;
		self
	}

	pub fn pos(&mut self, pos: Position) -> &mut Self {
		self.pos = pos;
		self
	}

	pub fn title(&mut self, title: &str) -> &mut Self {
		self.title = title.to_owned();
		self
	}

	pub fn fullscreen(&mut self) -> &mut Self {
		self.fullscreen = true;
		self
	}

	pub fn build(&mut self) -> Window {
		#[cfg(unix)]
		todo!();

		#[cfg(windows)]
		return Window {
			handle: os::windows::create_window(self.size, self.pos, self.title.as_str(), self.fullscreen),
			open: true
		};
	}
}

pub struct Window {
	handle: WindowHandle,
	open: bool
}

impl Window {
	pub fn new() -> WindowBuilder {
		let mut window_builder = WindowBuilder { 
			size: Size::new(800, 600),
			pos: Position::new(0, 0),
			title: "My Window".to_owned(),
			fullscreen: false
		};

		let screen_size = screen::get_size(ScreenType::Main);

		window_builder.pos = Position::new(((screen_size.width - window_builder.size.width) / 2) as i32, ((screen_size.height - window_builder.size.height) / 2) as i32);
		window_builder
	}

	pub fn handle(&self) -> WindowHandle {
		self.handle
	}

	pub fn pos(&self) -> Position {
		#[cfg(unix)]
		todo!();

		#[cfg(windows)]
		return os::windows::get_position(self.handle);
	}

	pub fn size(&self) -> Size {
		#[cfg(unix)]
		todo!();

		#[cfg(windows)]
		return os::windows::get_size(self.handle);
	}

	pub fn set_pos(&mut self, pos: Position) {
		#[cfg(unix)]
		todo!();

		#[cfg(windows)]
		os::windows::set_position(self.handle, pos);
	}

	pub fn set_size(&mut self, size: Size) {
		#[cfg(unix)]
		todo!();

		#[cfg(windows)]
		os::windows::set_size(self.handle, size);
	}

	pub fn open(&self) -> bool {
		self.open
	}

	pub fn poll_event(&self, event: &mut Event) -> bool {
		#[cfg(unix)]
		todo!();

		#[cfg(windows)]
		return os::windows::poll_event(self.handle, event);
	}

	pub fn close(&mut self) {
		self.open = false;
	}
}

impl Drop for Window {
	fn drop(&mut self) {
		#[cfg(unix)]
		todo!();

		#[cfg(windows)]
		os::windows::destroy_window(self.handle);
	}
}