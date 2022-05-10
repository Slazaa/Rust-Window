#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Event {
	Close,
	KeyDown(char),
	KeyUp(char),
	None
}