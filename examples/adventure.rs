extern crate bhavana_engine;

use bhavana_engine::conf::WindowSettings;
use bhavana_engine::system::SystemBuilder;

fn main() {
	let mut system = SystemBuilder::new()
		.window_settings(WindowSettings {
			width: 1920 / 2,
			height: 1080 / 2,
			title: "Trit's Adventure",
		})
		.build()
		.unwrap();
}
