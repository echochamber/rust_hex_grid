extern crate rand;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate find_folder;
extern crate sprite;
extern crate gfx_device_gl;
extern crate piston;

mod board;
mod view;

use opengl_graphics::OpenGL;
use piston::event::{Event, Events, RenderEvent};
use piston::input::{Button, Input};
use piston::window::WindowSettings;
use piston_window::{PistonWindow, clear};
use board::*;
use view::*;

fn main() {

	let opengl = OpenGL::V3_2;

	let (width, height) = (1280, 720);
    let window: PistonWindow =
        WindowSettings::new("piston: HexGrid", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .into();

    let app = App::new(ViewSettings::new());

    for e in window {
		e.draw_2d(|c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
    	});

    	e.draw_2d(|c, g| {
    		app.render(c, g);
    	});
	}
}
