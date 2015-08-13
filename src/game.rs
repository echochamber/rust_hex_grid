use piston_window::*;
use hex_board::*;
use hex2d::*;

pub struct GameOptions {
	pub grid_size: i32,
	pub hex_size: f32
}

pub struct Game {
	options: GameOptions,
	board: HexBoard,
	mouse_coords: (f32, f32),
	offset: f64
}

impl Game {
	pub fn new(options: GameOptions) -> Game {
		Game {
			board: HexBoard::new_radius(options.grid_size, Spacing::PointyTop(options.hex_size)),
			options: options,
			mouse_coords: (0.0, 0.0),
			offset: 200.0
		}
	}

	pub fn render(&mut self, c: Context, g: &mut G2d) {
        clear([1.0, 1.0, 1.0, 1.0], g);
        //render_board(selected, grid_size, spacing, c.trans(200.0, 200.0), g);
        render_hex_board(&self.board, c.trans(self.offset, self.offset), g);
	}

	pub fn handle_input(&mut self, input: Input) {
		match input {
	        Input::Move(Motion::MouseCursor(x, y)) => {
	            self.mouse_coords = (x as f32, y as f32);
	        },
	        Input::Press(Button::Mouse(_)) => {
	            let coord = axial_pixel_to_hex(self.mouse_coords.0 - self.offset as f32, self.mouse_coords.0 - self.offset as f32, Spacing::PointyTop(self.options.hex_size));
	            self.board.select(coord);
	            println!("clicked: {:?}", coord);
	            println!("coordinates: {:?}", self.mouse_coords);
	        },
	        Input::Press(Button::Keyboard(key)) => {
	            let coord = axial_pixel_to_hex(self.mouse_coords.0 - self.offset as f32, self.mouse_coords.1 - self.offset as f32, Spacing::PointyTop(self.options.hex_size));
	            let triangle_op = self.board.get_triangle(coord);
	            let mut triangle_id = match triangle_op {
	                Some(i) => { i },
	                None => { 0 }
	            };
	            match key {
	                Key::Up => {
	                    if triangle_op.is_some() {
	                        self.board.set_triangle(coord, Some(triangle_id + 1));
	                    }
	                },
	                Key::Down => {
	                    if triangle_op.is_some() {
	                        self.board.set_triangle(coord, Some(triangle_id - 1));
	                    }
	                },
	                Key::Space => {
	                    if (triangle_op.is_some()) {
	                        self.board.set_triangle(coord, None);
	                    } else {
	                        self.board.set_triangle(coord, Some(3));
	                    }
	                    
	                }
	                _ => {}
	            };
	        }
	        _ => {}
	    }
	}
}