use board::*;

pub struct Vec2f {
	pub x: f64,
	pub y: f64
}

pub struct ViewSettings {
	pub window_size: Vec2f,
	pub cell_size: f64,
	pub font_size: u32,
	pub text_offset: Vec2f,
	pub grid_size: u8
}

impl ViewSettings {
	pub fn new() -> ViewSettings {
		ViewSettings {
			window_size: Vec2f{ x: 1280.0, y: 720.0 },
			cell_size: 25.0,
			font_size: 64,
			text_offset: Vec2f{ x: 30.0, y: 75.0 },
			grid_size: 7
		}
	}
}

pub struct App {
	settings: ViewSettings,
	pub grid: HexGrid
}

impl App {
	pub fn new(settings: ViewSettings) -> App {
		App {
			grid: HexGrid::new(settings.grid_size, settings.cell_size),
			settings: settings
		}
	}
}