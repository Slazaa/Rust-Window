# Rust - Window Manager
A cross-platform window manager

## Featues
- Screen
	- Size
- Window
	- Create
	- Handle
	- Position
	- Size

## Examples
```rs
use window_manager::window::Window;
use window_manager::event::Event;

fn main() {
	let mut window = Window::new() // Create new window
		.title("Test")
		.build();

	while window.open() { // Keep the window open until it is manually closed
		let mut event = Event::None;

		while window.poll_event(&mut event) { // Processe every events on each frame
			match event {
				Event::Close => window.close(), // Close the window if the close event is triggered
				_ => ()
			}
		}
	}
}
```

## Libraries used
* [winapi](https://github.com/retep998/winapi-rs)