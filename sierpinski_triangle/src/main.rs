extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

struct App {
    gl: GlGraphics,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);
        });
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let window: Window = WindowSettings::new(
        "Sierpinski-Triangle",
        [1000, 800])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App{gl: GlGraphics::new(opengl)};

    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
    }
}
