mod os;

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
	fullscreen: bool,
	visible: bool
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

	pub fn visible(&mut self) -> &mut Self {
		self.visible = true;
		self
	}

	pub fn build(&mut self) -> Result<Window, String> {
		#[cfg(unix)]
		todo!();

		#[cfg(windows)]
		return Ok(Window {
			handle: match os::windows::create_window(self.size, self.pos, self.title.as_str(), self.fullscreen, self.visible) {
				Ok(x) => x,
				Err(e) => return Err(e)
			}
		});
	}
}

pub struct Window {
	handle: WindowHandle
}

impl Window {
	pub fn new() -> WindowBuilder {
		let mut window_builder = WindowBuilder { 
			size: Size::new(800, 600),
			pos: Position::new(0, 0),
			title: "My Window".to_owned(),
			fullscreen: false,
			visible: false
		};

		let screen_size = screen::get_size(ScreenType::Main);

		window_builder.pos = Position::new(((screen_size.width - window_builder.size.width) / 2) as i32, ((screen_size.height - window_builder.size.height) / 2) as i32);
		window_builder
	}

	pub fn handle(&self) -> &WindowHandle {
		&self.handle
	}
}