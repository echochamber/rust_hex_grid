#![feature(append)]

extern crate piston_window;
extern crate opengl_graphics;
extern crate piston;
extern crate hex2d;

use hex_board::*;
use piston_window::*;

mod hex_board;

fn main() {
    

    // Variables for rendering 2d hex board. Could be put into a struct called HexBoardOptions I suppose.
    let grid_size = 4;
    let hex_size = 20.0;
    let spacing = hex2d::Spacing::PointyTop(hex_size);

    // Variables for the piston window
    let opengl = OpenGL::V3_2;
    let (width, height) = (800, 800);
    let window: PistonWindow =
        WindowSettings::new("piston: HexGrid", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .into();

    // Where the mouse was last moved to.
    let mut mouse_coords = (0i32, 0i32);
    // Coordinates of the selected hex (Last clicked hex).
    let mut selected = Some(hex2d::Coordinate::new(0,0));

    for e in window {
        match e.event {
            Some(Event::Render(render_args)) => {
                e.draw_2d(|c, g| {
                    clear([1.0, 1.0, 1.0, 1.0], g);
                    render_board(c.trans(200.0, 200.0), g, selected, grid_size, spacing);
                });
            },
            Some(Event::Input(Input::Move(Motion::MouseCursor(x, y)))) => {
                mouse_coords = (x as i32 - 200, y as i32 - 200);
            },
            Some(Event::Input(Input::Press(Button::Mouse(_)))) => {
                let coord = axial_pixel_to_hex(mouse_coords.0 as f32, mouse_coords.1 as f32, spacing);
                if coord.x.abs() > grid_size || coord.y.abs() > grid_size {
                    selected = None;
                } else {
                    selected = Some(coord);
                }
                println!("clicked: {:?}", coord);
                println!("selected: {:?}", selected);
                println!("coordinates: {:?}", mouse_coords);
            }
            _ => {}
        }
    }
}