extern crate zazen_engine;

use zazen_engine::conf::WindowSettings;
use zazen_engine::context::ContextBuilder;

fn main() {
	let _context = ContextBuilder::new()
					.window_settings(WindowSettings{
						width: 900,
						height: 600,
						title: "Trit's Adventure"
					})
					.build();
}