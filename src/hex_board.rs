#![feature(append)]
use hex2d::*;
use piston_window::*;

pub fn render_board(c: Context, g: &mut G2d, selected: Option<Coordinate>, grid_size: i32) {
	let coord: Coordinate = Coordinate::new(12, 12);

	
	let origin = Coordinate::new(0, 0);
	// let mut board: Vec<Position> = Vec::new();
	// board.push(Position::new(origin, Direction::YZ));

	let mut board: Vec<Coordinate> = Vec::new();
	board.push(origin);
	for i in 1..grid_size + 1 {
		board.append(&mut origin.ring(i, Spin::CCW(XY)));
	}

	for coordinate in board.iter() {
		if selected.is_some() && coordinate.clone() == selected.unwrap() {
			draw_hex(coordinate, 20.0, [0.75, 0.25, 0.5, 1.0], c.trans(200.0, 200.0), g);	
		} else {
			draw_hex(coordinate, 20.0, [0.25; 4], c.trans(200.0, 200.0), g);	
		}
		
	}
}

// Todo make this work correctly for flat top
pub fn draw_hex(coordinate: &Coordinate<i32>, size: f64, color: [f32; 4], c: Context, g: &mut G2d) {
	let width = size * 1.732058;
	let height = size * 2.0;

	let (x,y) = coordinate.to_pixel_float(Spacing::PointyTop(size as f32));
	let hex_vertices = hex_vertices([0.0, 0.0], size);
	polygon(
		color,
		&hex_vertices,
		c.transform.trans(x as f64, y as f64),
		g
	);

	// Draw the border around each hex, 6 lines
	// Improve?
	let vertex_iter = (&hex_vertices).iter().enumerate();
	for (i, vertex) in vertex_iter {

		// Assignment using destructuring is unstable for arrays?
		let (x0, y0) = (vertex[0], vertex[1]);

		let (x1, y1) = if i != 5 {
			(hex_vertices[i + 1][0], hex_vertices[i + 1][1])
		} else {
			(hex_vertices[0][0], hex_vertices[0][1])
		};

		let line = Line::new([0.0, 0.0, 0.0, 0.5], 0.5)
			.shape(line::Shape::Round);

		Line::draw(
			&line,
			[x0, y0, x1, y1],
			&c.draw_state,
			c.transform.trans(x as f64, y as f64),
			g
		);
	}
}

// Get the vertex for the polygon
fn hex_vertices(center: [f64; 2], size: f64) -> [[f64; 2]; 6] {
	let mut vertices: [[f64; 2]; 6] = [[0.0; 2]; 6];
	for i in 0..6usize {
		vertices[i] = hex_vertex(center, size, i as u32);
	}

	vertices
}

// Get one of the vertexes of a hexagon ( 0 <= i < 6)
fn hex_vertex(center: [f64; 2], size: f64, i: u32) -> [f64; 2] {
	let angle_deg: f64 = 60.0 * i as f64 + 30.0;
	let angle_rad: f64 = ::std::f64::consts::PI / 180.0 * angle_deg;
	return [
		center[0] + size * angle_rad.cos(),
		center[1] + size * angle_rad.sin()
	];
}

/// Pixel coordinates tend to be in axial form, so this function 
/// is likely more convenient than the one provided by hex2d-rust
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