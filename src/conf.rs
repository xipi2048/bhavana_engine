pub(crate) struct Conf {
	window_settings: WindowSettings
}

impl Conf {
	pub(crate) fn new() -> Self {
		Self {
			window_settings: WindowSettings::default()
		}
	}
}

pub struct WindowSettings {
	pub height: u16,
	pub width: u16,
	pub title: &'static str
}

impl Default for WindowSettings {
	fn default() -> Self {
		WindowSettings {
			height: 1280,
			width: 720,
			title: "Window"
		}
	}
}