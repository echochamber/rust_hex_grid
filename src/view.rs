use board::*;
use opengl_graphics::GlGraphics;
use piston_window::*;
use app::*;

impl Renderable for App {
	fn render(&self, c: Context, g: &mut G2d) {
		let mut grid_context = c.clone();
		self.grid.render(grid_context, g);
	}
}

impl Renderable for HexGrid {
	fn render(&self, c: Context, g: &mut G2d) {
		let width = self.hex_size * 1.732058;
		let height = self.hex_size * 2.0;


		// Since the origin (0.0, 0.0) is at the center of the first hex, we need to 
		// shift it by half a height and half a width so its entirely in the viewport.
		let transform  = c.transform.trans(width / 2.0, height / 2.0);

		// // Draw lines for debugging
		// let newline = Line::new([0.0, 0.0, 0.0, 0.5], 1.0);

		// Line::draw(
		// 	&newline,
		// 	[0.0, -self.hex_size, 0.0, self.hex_size],
		// 	&c.draw_state,
		// 	transform,
		// 	g
		// );

		// Line::draw(
		// 	&newline,
		// 	[-width / 2.0, 0.0, width / 2.0, 0.0],
		// 	&c.draw_state,
		// 	transform,
		// 	g
		// );
		// let rddd = 1;
		// let qddd = 0;
		// let shift = (rddd % 2) as f64 * width * 0.5;

		// let xdsada = qddd as f64 * width + shift;
		// let ydsada = rddd as f64 * height;

		// Line::draw(
		// 	&newline,
		// 	[-500.0, 0.0, 500.0, 0.0],
		// 	&c.draw_state,
		// 	transform.trans(xdsada, ydsada),
		// 	g
		// );

		// Line::draw(
		// 	&newline,
		// 	[0.0, -500.0, 0.0, 500.0],
		// 	&c.draw_state,
		// 	transform.trans(xdsada, ydsada),
		// 	g
		// );

		for q in 0..self.grid_size {
			for r in 0..self.grid_size {
				// Amount to shift each odd row so the hexagons 
				// form a grid without overlap or gaps.
				let shift = (r % 2) as f64 * self.hex_size * 0.5 * 1.732058;

				let x = q as f64 * width + shift;
				let y = r as f64 * height * 0.75;
				let hex_vertices = hex_vertices([0.0, 0.0], self.hex_size);
				polygon(
					[0.25; 4],
					&hex_vertices,
					transform.trans(x, y),
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
						transform.trans(x, y),
						g
					);
				}
			}
		}
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

// Maybe this should take an AppRenderContext struct (with information 
// like if the hex is selected). Alternatively I could make a RenderInfo
// trait that
// since having the hex cells be a struct might be preferable once we
// start rendering stuff in them. 
pub trait Renderable {
	fn render(&self, c: Context, g: &mut G2d);
}

// Returns the center of the
pub fn in_hex_grid(grid: &HexGrid, point: ::Point) -> Option<(HexCoord, ::Point)> {
	for (hex_coord, center) in grid.hex_centers() {
		if in_hex(center, grid.hex_size, point) {
			return Some((hex_coord, center));
		}
	}

	return None;
}

pub fn in_hex(center: ::Point, size: f64, point: ::Point) -> bool {
	let vertices = hex_vertices(center, size);
	let mut j = 0;

	let mut result: bool = false;
	for i in 0..6 {
		j = i;
		if (vertices[i][1] > point[1]) != (vertices[j][1] > point[1]) && 
			(point[0] < (vertices[j][0] - vertices[i][1]) * (point[1] - vertices[i][1]) / (vertices[j][1] - vertices[i][1]) + vertices[i][0]) {
				result = !result;
		}
	}

	return result;
}