# Rust - Window
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
use window::window::Window;
use window::event::Event;

fn main() {
	let mut window = Window::new() // Creates new window
		.title("Test")
		.build();

	while window.open() { // Keeps the window open until it is manually closed
		let mut event = Event::None;

		while window.poll_event(&mut event) { // Processes every events on each frame
			match event {
				Event::KeyDown(key) => println!("'{}' has been pressed!", key), // Prints the key pressed
				Event::Close => window.close(), // Closes the window if the close event is triggered
				_ => ()
			}
		}
	}
}
```

## Libraries used
* [winapi](https://github.com/retep998/winapi-rs)
