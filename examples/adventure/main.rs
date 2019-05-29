extern crate bhavana_engine;

use bhavana_engine::conf::WindowSettings;
use bhavana_engine::system::SystemBuilder;

fn main() {
	let _system = SystemBuilder::new()
		.window_settings(WindowSettings {
			title: "Trit's Adventure",
			..WindowSettings::default()
		})
		.build()
		.unwrap();
}
