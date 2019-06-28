extern crate bhavana_engine as bheng;

use bheng::{
	conf::WindowSettings,
	system::SystemBuilder
};

mod game;

fn main() {
	let mut system = SystemBuilder::new()
		.window_settings(WindowSettings {
			title: "Trit's Adventure",
			..WindowSettings::default()
		})
		.build()
		.unwrap();

	system.run(game::Game::new());
}
