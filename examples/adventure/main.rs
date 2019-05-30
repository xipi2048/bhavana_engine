extern crate bhavana_engine;

use bhavana_engine::conf::WindowSettings;
use bhavana_engine::system::SystemBuilder;

fn main() {
	let mut system = SystemBuilder::new()
		.window_settings(WindowSettings {
			title: "Trit's Adventure",
			..WindowSettings::default()
		})
		.build()
		.unwrap();

	system.run_forever(|event| {
		match event {
			bhavana_engine::Event::WindowEvent {
				event: bhavana_engine::WindowEvent::CloseRequested,
				..
			} => bhavana_engine::ControlFlow::Break,
			_ => bhavana_engine::ControlFlow::Continue
		}		
	});
}
