#[derive(Clone)]
pub struct Conf {
	pub window_settings: WindowSettings
}

#[derive(Clone)]
pub struct WindowSettings {
	pub width: u32,
	pub height: u32,
	pub title: &'static str
}

impl Default for Conf {
	fn default () -> Self {
		Self {
			window_settings: WindowSettings::default()
		}
	}
}

impl Default for WindowSettings {
	fn default() -> Self {
		WindowSettings {
			width: 1280,
			height: 720,
			title: "Window"
		}
	}
}