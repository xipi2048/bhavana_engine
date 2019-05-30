extern crate bhavana_engine as bheng;

use bheng::conf::WindowSettings;
use bheng::system::SystemBuilder;

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
			bheng::Event::WindowEvent {
				event: bheng::WindowEvent::CloseRequested,
				..
			} => bheng::ControlFlow::Break,
			_ => bheng::ControlFlow::Continue
		}		
	});
}
