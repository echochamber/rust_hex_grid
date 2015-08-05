extern crate rand;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate find_folder;
extern crate sprite;
extern crate gfx_device_gl;
extern crate piston;

use opengl_graphics::OpenGL;
use piston::window::WindowSettings;
use piston_window::{PistonWindow, clear};
use piston::input::Input;
use piston::event::Event;

use board::*;
use view::*;
use app::*;

mod board;
mod view;
mod app;

fn main() {
	let opengl = OpenGL::V3_2;

	let (width, height) = (1280, 720);
    let window: PistonWindow =
        WindowSettings::new("piston: HexGrid", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .into();

    let mut app = App::new(ViewSettings::new());

    for e in window {
        match e.event {
            Some(Event::Render(render_args)) => {
                e.draw_2d(|c, g| {
                    clear([1.0, 1.0, 1.0, 1.0], g);
                    app.render(c, g);
                });
            },
            Some(Event::Input(input)) => {
                app.handle_input(&input);
                println!("{:?}", input);
            },
            Some(Event::Idle(_)) => {
                // Do nothing
            },
            Some(Event::Update(_)) => {
                // Do nothing
            },
            Some(Event::AfterRender(_)) => {
                // Do nothing
            },
            _ => { println!("{:?}", &e.event); }
        }
	}
}

pub type Point = [f64; 2];