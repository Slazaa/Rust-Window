mod os;

use crate::context::{self, DeviceContextHandle};
use crate::event::Event;
use crate::utils::{Interface, Position, Size};
use crate::screen::{self, ScreenType};

#[cfg(unix)]
pub type WindowHandle = self::os::unix::WindowHandle;

#[cfg(windows)]
pub type WindowHandle = self::os::windows::WindowHandle;

#[derive(Copy, Clone)]
pub struct Style {
	pub visible: bool,
	pub border: bool,
	pub titlebar: bool,
	pub close: bool,
	pub maximize: bool,
	pub minimize: bool,
	pub resize: bool,
	pub fullscreen: bool
}

impl Default for Style {
	fn default() -> Self {
		Self {
			visible: true,
			border: true,
			titlebar: true,
			close: true,
			maximize: true,
			minimize: true,
			resize: false,
			fullscreen: false
		}
	}
}

pub struct DeviceContext {
    pub handle: DeviceContextHandle,
    pub interface: Interface
}

pub struct WindowBuilder {
	size: Size,
	pos: Position,
	title: String,
	style: Style,
	context_interface: Option<Interface>
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

	pub fn style(&mut self, style: Style) -> &mut Self {
		self.style = style;
		self
	}

	pub fn context(&mut self, interface: Interface) -> &mut Self {
		self.context_interface = Some(interface);
		self
	}

	pub fn build(&mut self) -> Result<Window, String> {
		#[cfg(unix)]
		let handle = os::unix::create_window(self.size, self.pos, self.title.as_str(), self.style);

		#[cfg(windows)]
		let handle = os::windows::create_window(self.size, self.pos, self.title.as_str(), self.style);

		let handle = match handle {
			Ok(x) => x,
			Err(e) => return Err(e)
		};

		let mut context = None;

		if let Some(interface) = self.context_interface {
			context = Some(DeviceContext {
                handle: match context::create_context(handle, interface) {
                    Ok(x) => x,
                    Err(e) => return Err(e)
                },
                interface
			});
		}

		Ok(Window {
			handle,
			context,
			open: true
		})
	}
}

pub struct Window {
	handle: WindowHandle,
	context: Option<DeviceContext>,
	open: bool
}

impl Window {
	pub fn new() -> WindowBuilder {
		let mut window_builder = WindowBuilder { 
			size: Size::new(800, 600),
			pos: Position::new(0, 0),
			title: "New Window".to_owned(),
			style: Style::default(),
			context_interface: None
		};

		let screen_size = screen::get_size(ScreenType::Main);

		window_builder.pos = Position::new(((screen_size.width - window_builder.size.width) / 2) as i32, ((screen_size.height - window_builder.size.height) / 2) as i32);
		window_builder
	}

	pub fn handle(&self) -> WindowHandle {
		self.handle
	}

	pub fn context(&self) -> Option<&DeviceContext> {
		match &self.context {
            Some(x) => Some(&x),
            None => None
        }
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
		if let Some(context) = &self.context {
			context::release_context(self.handle, context.handle, context.interface);
		}

        #[cfg(unix)]
        os::unix::destroy_window(self.handle);

        #[cfg(windows)]
        os::windows::destroy_window(self.handle);
	}
}
