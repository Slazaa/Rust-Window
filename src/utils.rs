#[derive(Clone, Copy, Debug)]
pub struct Position {
	pub x: i32,
	pub y: i32
}

impl Position {
	pub fn new(x: i32, y: i32) -> Self {
		Self {
			x,
			y
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Size {
	pub width: u32,
	pub height: u32
}

impl Size {
	pub fn new(width: u32, height: u32) -> Self {
		Self {
			width,
			height
		}
	}
}