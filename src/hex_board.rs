#![feature(append)]
use hex2d::*;
use piston_window::*;

// Draw an entire grid of hexagons
// grid size is the radius of hexagons surrounding the origin.
pub fn render_board(selected: Option<Coordinate>, grid_size: i32, spacing: Spacing, c: Context, g: &mut G2d) {
	let origin = Coordinate::new(0, 0);
	let mut board: Vec<Coordinate> = Vec::new();
	board.push(origin);
	for i in 1..grid_size + 1 {
		board.append(&mut origin.ring(i, Spin::CCW(XY)));
	}

	for coordinate in board.iter() {
		if selected.is_some() && coordinate.clone() == selected.unwrap() {
			render_hex(coordinate, spacing, [0.75, 0.25, 0.5, 1.0], c, g);	
		} else {
			render_hex(coordinate, spacing, [0.25; 4], c, g);
		}
	}
}

pub type Point =  [f64; 2];
pub type Triangle = [Point; 3];
pub type Hexagon = [Point; 6];

pub struct HexBoard {
	hexes: Vec<PixelHex>
}

impl HexBoard {
	pub fn new_radius(radius: i32, spacing: Spacing) -> HexBoard {

		let origin = Coordinate::new(0, 0);
		let mut board: Vec<PixelHex> = Vec::new();
		board.push(PixelHex::new(origin, spacing, None));
		for i in 1..radius + 1 {
			board.append(
				&mut origin.ring(i, Spin::CCW(XY)).iter().map(|&x|
					PixelHex::new(x, spacing, None)
				).collect::<Vec<PixelHex>>()
			);
		}

		HexBoard {
			hexes: board
		}
	}

	pub fn select(&mut self, coordinate: Coordinate<i32>) {
		match self.hexes.iter_mut().find(|x| x.coordinate == coordinate) {
			Some(pixel_hex) => {
				pixel_hex.state = if pixel_hex.state == PixelHexState::Selected {
					PixelHexState::Default
				} else {
					PixelHexState::Selected
				};
			},
			None => {}
		};
	}

	pub fn set_triangle(&mut self, coordinate: Coordinate<i32>, filled_triangle: Option<i32>) {
		match self.hexes.iter_mut().find(|x| x.coordinate == coordinate) {
			Some(pixel_hex) => {
				pixel_hex.filled_triangle = filled_triangle;
			},
			None => {}
		};
	}

	pub fn get_triangle(&self, coordinate: Coordinate<i32>) -> Option<i32> {
		match self.hexes.iter().find(|x| x.coordinate == coordinate) {
			Some(pixel_hex) => {
				pixel_hex.filled_triangle
			},
			None => { None }
		}
	}
}

pub fn render_hex_board(board: &HexBoard, c: Context, g: &mut G2d) {
	for pixel_hex in board.hexes.iter() {
		render_pixel_hex(pixel_hex, c, g);
	}
}

struct PixelHex {
	pub vertices: Hexagon,
	pub coordinate: Coordinate<i32>,
	pub spacing: Spacing,
	pub center: Point,
	pub state: PixelHexState,
	pub rotation: i32,
	pub filled_triangle: Option<i32>
}

impl PixelHex {
	pub fn new(coordinate: Coordinate, spacing: Spacing, filled_triangle: Option<i32>) -> PixelHex {
		let (x, y) = coordinate.to_pixel_float(spacing);
		let center = [x as f64, y as f64];
		PixelHex {
			vertices: hex_vertices(center, spacing),
			coordinate: coordinate,
			spacing: spacing,
			center: center,
			state: PixelHexState::Default,
			rotation: 0,
			filled_triangle: filled_triangle
		}
	}
}

fn render_pixel_hex(hex: &PixelHex, c: Context, g: &mut G2d) {
	let color = match hex.state {
		PixelHexState::Selected => { [0.75, 0.25, 0.5, 1.0] },
		PixelHexState::Default => { [0.25; 4] }
	};

	polygon(
		color,
		&hex.vertices,
		c.transform,
		g
	);

	let x = hex.center[0];
	let y = hex.center[1];
	let vertex_iter = (&hex.vertices).iter().enumerate();
	for (i, vertex) in vertex_iter {

		// Assignment using destructuring is unstable for arrays?
		let (x0, y0) = (vertex[0], vertex[1]);
		let current = i % 6;
		let next = (i + 1) % 6;
		let (x1, y1) = (hex.vertices[next][0], hex.vertices[next][1]);

		let line = Line::new([0.0, 0.0, 0.0, 0.5], 0.5)
			.shape(line::Shape::Round);

		Line::draw(
			&line,
			[x0, y0, x1, y1],
			&c.draw_state,
			c.transform,
			g
		);
	}

	match hex.filled_triangle {
		Some(i) => {
			render_triangle_hex(hex.center, hex.spacing, i, [0.4, 0.4, 0.8, 1.0], c, g);
		},
		None => {}
	};
}

#[derive(Eq, PartialEq)]
enum PixelHexState {
	Selected,
	Default
}

pub fn render_triangle_hex(center: Point, spacing: Spacing, i: i32, color: [f32; 4], c: Context, g: &mut G2d) {

	let triangle =  hex_triangle(center, spacing, i);

	polygon(
		color,
		&triangle,
		c.transform,
		g
	);

	let line = Line::new([0.0, 0.0, 0.0, 0.5], 0.5)
		.shape(line::Shape::Round);

	Line::draw(
		&line,
		[triangle[0][0], triangle[0][1], triangle[1][0], triangle[1][1]],
		&c.draw_state,
		c.transform,
		g
	);

	Line::draw(
		&line,
		[triangle[1][0], triangle[1][1], triangle[2][0], triangle[2][1]],
		&c.draw_state,
		c.transform,
		g
	);

	Line::draw(
		&line,
		[triangle[0][0], triangle[0][1], triangle[2][0], triangle[2][1]],
		&c.draw_state,
		c.transform,
		g
	);
}

// Draw a single hexagon and its border.
pub fn render_hex(coordinate: &Coordinate<i32>, spacing: Spacing, color: [f32; 4], c: Context, g: &mut G2d) {
	let size = match spacing {
		Spacing::PointyTop(size) => { size as f64 },
		Spacing::FlatTop(size) => { size as f64 }
	};

	let width = size * 1.732058;
	let height = size * 2.0;

	// (x,y) are the pixel coordinates at the center of the hexagon being rendered
	let (x,y) = coordinate.to_pixel_float(spacing);
	let hex_vertices = hex_vertices([x as f64, y as f64], spacing);
	polygon(
		color,
		&hex_vertices,
		c.transform,
		g
	);

	// Draw the border around each hex, 6 lines
	// Improve?
	let vertex_iter = (&hex_vertices).iter().enumerate();
	for (i, vertex) in vertex_iter {

		let current = i % 6;
		let next = (i + 1) % 6;
		// Assignment using destructuring is unstable for arrays?
		let (x0, y0) = (vertex[0], vertex[1]);

		let (x1, y1) = (hex_vertices[next][0], hex_vertices[next][1]);

		let line = Line::new([0.0, 0.0, 0.0, 0.5], 0.5)
			.shape(line::Shape::Round);

		Line::draw(
			&line,
			[x0, y0, x1, y1],
			&c.draw_state,
			c.transform,
			g
		);
	}
}

/// Remove this method and use from_pixel once PR is merged in hex2d-rs
/// Pixel coordinates tend to be in axial form, so this function 
/// is likely more convenient than the one provided by hex2d-rs
pub fn axial_pixel_to_hex(x: f32, y: f32, spacing: Spacing) -> Coordinate {
	match spacing {
		Spacing::PointyTop(size) => {
			let q = (x * 3f32.sqrt()/3f32 - y / 3f32) / size;
    		let r = y * 2f32/3f32 / size;
    		return Coordinate::from_round(q, -r -q);
		},
		Spacing::FlatTop(size) => {
			let q = x * 2f32/3f32 / size;
			let r = (-x / 3f32 + 3f32.sqrt()/3f32 * y) / size;
			return Coordinate::from_round(q, -r -q);
		}
	}
}

// Get all the vertices for a hexagon
fn hex_vertices(center: Point, spacing: Spacing) -> Hexagon {
	let mut vertices: [Point; 6] = [[0.0; 2]; 6];
	for i in 0..6usize {
		vertices[i] = hex_vertex(center, spacing, i as i32);
	}

	vertices
}

// Get the coordinates of a triangle in a hex
fn hex_triangle(center: Point, spacing: Spacing, i: i32) -> Triangle{

	let current = i % 6;
	let other = (i + 1) % 6;

	return [
		center,
		hex_vertex(center, spacing, other),
		hex_vertex(center, spacing, current)
	];
}

// Get one of the vertexes of a hexagon ( 0 <= i < 6)
fn hex_vertex(center: Point, spacing: Spacing, i: i32) -> Point {
	let current = i % 6;
	match spacing {
		Spacing::PointyTop(size) => {
			let angle_deg: f64 = 60.0 * current as f64 + 30.0;
			let angle_rad: f64 = ::std::f64::consts::PI / 180.0 * angle_deg;
			return [
				center[0] + size as f64 * angle_rad.cos(),
				center[1] + size as f64 * angle_rad.sin()
			];
		},
		Spacing::FlatTop(size) => {
			let angle_deg: f64 = 60.0 * current as f64;
			let angle_rad: f64 = ::std::f64::consts::PI / 180.0 * angle_deg;
			return [
				center[0] + size as f64 * angle_rad.cos(),
				center[1] + size as f64 * angle_rad.sin()
			];
		}
	};
}