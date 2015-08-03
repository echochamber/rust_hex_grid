use board::*;
use opengl_graphics::GlGraphics;
use piston_window::*;
use app::*;

impl Renderable for App {
	fn render(&self, c: Context, g: &mut G2d) {
		self.grid.render(c, g);
	}
}

impl Renderable for HexGrid {
	fn render(&self, c: Context, g: &mut G2d) {

		// Just some small amount so its all actually 
		// inside the window while testing
		let transform = c.transform.trans(200.0, 200.0);

		for q in 0..self.grid_size {
			for r in 0..self.grid_size {
				// Amount to shift each odd row so the hexagons 
				// form a grid without overlap or gaps.
				let shift = (r % 2) as f64 * self.hex_size * 0.5 * 1.732058;

				let x = q as f64 * (self.hex_size * 1.732058) + shift;
				let y = r as f64 * self.hex_size * 1.5;
				let hex_vertices = hex_vertices([0.0, 0.0], self.hex_size);
				polygon(
					[0.25; 4],
					&hex_vertices,
					transform.trans(x, y),
					g
				);

				// Draw the border around each hex, 6 lines
				// Improve?
				let vertex_iter = (&hex_vertices).iter().enumerate().peekable();
				for (i, vertex) in vertex_iter {

					// Assignment using destructuring is unstable for arrays?
					let (x0, y0) = (vertex[0], vertex[1]);

					let (x1, y1) = if i != 5 {
						(hex_vertices[i + 1][0], hex_vertices[i + 1][1])
					} else {
						(hex_vertices[0][0], hex_vertices[0][1])
					};

					let line = Line::new([0.0, 0.0, 0.0, 0.5], 1.5);

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