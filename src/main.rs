#![feature(append)]

extern crate piston_window;
extern crate opengl_graphics;
extern crate piston;
extern crate hex2d;

use opengl_graphics::OpenGL;
use piston::window::WindowSettings;
use piston_window::{PistonWindow, clear};
use piston::input::{Input, Motion, Button};
use piston::event::Event;

use hex_board::*;

mod hex_board;

fn main() {
	let opengl = OpenGL::V3_2;

	let (width, height) = (800, 800);
    let window: PistonWindow =
        WindowSettings::new("piston: HexGrid", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .into();

    let mut mouse_coords = (0i32, 0i32);
    let mut selected: hex2d::Coordinate = hex2d::Coordinate::new(0,0);
    let grid_size = 4;

    for e in window {
        match e.event {
            Some(Event::Render(render_args)) => {
                e.draw_2d(|c, g| {
                    clear([1.0, 1.0, 1.0, 1.0], g);
                    render_board(c, g, Some(selected), grid_size);
                });
            },
            Some(Event::Input(Input::Move(Motion::MouseCursor(x, y)))) => {
                mouse_coords = (x as i32 - 200, y as i32 - 200);
            },
            Some(Event::Input(Input::Press(Button::Mouse(_)))) => {
                let coord = axial_pixel_to_hex(mouse_coords.0 as f32, mouse_coords.1 as f32, hex2d::Spacing::PointyTop(20.0));
                selected = coord;
                println!("clicked: {:?}", coord);
                println!("selected: {:?}", selected);
            }
            _ => {}
        }
    }
}