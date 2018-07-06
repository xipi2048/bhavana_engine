extern crate bhavana_engine;

use bhavana_engine::conf::WindowSettings;
use bhavana_engine::context::ContextBuilder;
use bhavana_engine::event;

fn main() {
	let mut context =  ContextBuilder::new()
					.window_settings(WindowSettings{
						width: 1920/2,
						height: 1080/2,
						title: "Trit's Adventure"
					})
					.build()
					.unwrap();
	
	event::run(&context);
}