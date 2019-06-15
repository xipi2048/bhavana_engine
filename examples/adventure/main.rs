extern crate bhavana_engine as bheng;

use bheng::conf::WindowSettings;
use bheng::system::SystemBuilder;
use bheng::state::StateManager;

fn main() {
	let mut system = SystemBuilder::new()
		.window_settings(WindowSettings {
			title: "Trit's Adventure",
			..WindowSettings::default()
		})
		.with_state_manager(Game {
			
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

struct Game {}

impl StateManager for Game {

}
