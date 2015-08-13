#![feature(append)]
use hex2d::*;
use piston_window::*;

// Draw an entire grid of hexagons
// grid size is the radius of hexagons surrounding the origin.
pub fn render_board(c: Context, g: &mut G2d, selected: Option<Coordinate>, grid_size: i32, spacing: Spacing) {
	
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
fn hex_vertices(center: [f64; 2], spacing: Spacing) -> [[f64; 2]; 6] {
	let mut vertices: [[f64; 2]; 6] = [[0.0; 2]; 6];
	for i in 0..6usize {
		vertices[i] = hex_vertex(center, spacing, i as u32);
	}

	vertices
}

// Get one of the vertexes of a hexagon ( 0 <= i < 6)
fn hex_vertex(center: [f64; 2], spacing: Spacing, i: u32) -> [f64; 2] {
	match spacing {
		Spacing::PointyTop(size) => {
			let angle_deg: f64 = 60.0 * i as f64 + 30.0;
			let angle_rad: f64 = ::std::f64::consts::PI / 180.0 * angle_deg;
			return [
				center[0] + size as f64 * angle_rad.cos(),
				center[1] + size as f64 * angle_rad.sin()
			];
		},
		Spacing::FlatTop(size) => {
			let angle_deg: f64 = 60.0 * i as f64;
			let angle_rad: f64 = ::std::f64::consts::PI / 180.0 * angle_deg;
			return [
				center[0] + size as f64 * angle_rad.cos(),
				center[1] + size as f64 * angle_rad.sin()
			];
		}
	};
}