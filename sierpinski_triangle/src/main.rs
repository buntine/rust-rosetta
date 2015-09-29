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
    triangles: Vec<[[f64; 2]; 4]>
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let triangles = &self.triangles;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);

            let transform = c.transform.trans(0.0, 0.0);

            for t in triangles {
              polygon(BLACK, t, transform, gl);
            }
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

    let triangles = vec![[[100.0, 200.0], [200.0, 200.0],
                          [150.0, 100.0], [100.0, 200.0]],
                          [[250.0, 200.0], [350.0, 200.0],
                          [300.0, 100.0], [250.0, 200.0]]];

    let mut app = App{gl: GlGraphics::new(opengl), triangles: triangles};

    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
    }
}
