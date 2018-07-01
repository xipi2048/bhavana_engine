extern crate zazen_engine;

use zazen_engine::{ContextBuilder, WindowSettings};

fn main() {
	let _context = ContextBuilder::new()
					.window_settings(WindowSettings{})
					.build();
}