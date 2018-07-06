pub(crate) struct Conf {
	window_settings: WindowSettings
}

impl Conf {
	pub(crate) fn new() -> Self {
		Self {
			window_settings: WindowSettings {
				height: 800,
				width: 600,
				title: "Window"
			}
		}
	}
}

pub struct WindowSettings {
	pub height: u16,
	pub width: u16,
	pub title: &'static str
}