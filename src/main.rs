#![feature(append)]

extern crate piston_window;
extern crate opengl_graphics;
extern crate piston;
extern crate hex2d;

use hex_board::*;
use game::*;
use piston_window::*;

mod hex_board;
mod game;

fn main() {
    let options = GameOptions {
        grid_size: 4,
        hex_size: 20.0
    };

    // Variables for the piston window
    let opengl = OpenGL::V3_2;
    let (width, height) = (800, 800);
    let window: PistonWindow =
        WindowSettings::new("piston: HexGrid", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .into();

    let mut game = Game::new(options);

    for e in window {
        match e.event {
            Some(Event::Render(render_args)) => {
                e.draw_2d(|c, g| {
                    clear([1.0, 1.0, 1.0, 1.0], g);
                    game.render(c, g);
                    //render_hex_board(&board, c.trans(200.0, 200.0), g);
                });
            },
            Some(Event::Input(Input::Resize(width, height))) => {
                println!("hit");
            },
            Some(Event::Input(input)) => {
                game.handle_input(input);
            },
            
            _ => {}
        }
    }
}