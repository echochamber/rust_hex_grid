use board::*;
use opengl_graphics::GlGraphics;
use piston_window::*;

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
	grid: HexGrid
}

impl App {
	pub fn new(settings: ViewSettings) -> App {
		App {
			grid: HexGrid::new(settings.grid_size, settings.cell_size),
			settings: settings
		}
	}
}

impl Renderable for App {
	fn render(&self, c: Context, g: &mut G2d) {
		self.grid.render(c, g);
	}
}

impl Renderable for HexGrid {
	fn render(&self, c: Context, g: &mut G2d) {
		let transform = c.transform.trans(200.0, 200.0);

		for q in 0..self.grid_size {
			for r in 0..self.grid_size {
				let shift = (r % 2) as f64 * self.hex_size * 0.5 * 1.732058;
				let x = q as f64 * (self.hex_size * 1.732058) + shift;
				let y = r as f64 * self.hex_size * 1.5;
				println!("{}", shift);
				polygon(
					[(r as f32 % 2.0), 1.0, 0.0, 1.0],
					&hex_corners([0.0, 0.0], self.hex_size),
					transform.trans(x, y),
					g
				);
			}
		}
	}	
}

fn hex_corners(center: [f64; 2], size: f64) -> [[f64; 2]; 6] {
	let mut corners: [[f64; 2]; 6] = [[0.0; 2]; 6];
	for i in 0..6usize {
		corners[i] = hex_corner(center, size, i as u32);
	}

	corners
}

fn hex_corner(center: [f64; 2], size: f64, i: u32) -> [f64; 2] {
	let angle_deg: f64 = 60.0 * i as f64 + 30.0;
	let angle_rad: f64 = ::std::f64::consts::PI / 180.0 * angle_deg;
	return [
		center[0] + size * angle_rad.cos(),
		center[1] + size * angle_rad.sin()
	];
}

pub trait Renderable {
	fn render(&self, c: Context, g: &mut G2d);
}