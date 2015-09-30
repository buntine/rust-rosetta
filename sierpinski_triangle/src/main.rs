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

fn sub_triangles(v: [[f64; 2]; 4]) -> Vec<[[f64; 2]; 4]> {
    let (a, b, c, d) = (v[0], v[1], v[2], v[3]); // Vector destructuring is not yet stable.

    vec![
        [[a[0] + ((b[0] - a[0]) / 2.0), a[1]],
         [b[0] - ((b[0] - c[0]) / 2.0), b[1] - ((b[1] - c[1]) / 2.0)],
         [a[0] + ((c[0] - a[0]) / 2.0), a[1] - ((a[1] - c[1]) / 2.0)],
         [a[0] + ((b[0] - a[0]) / 2.0), a[1]]],
        [[a[0], a[1]],
         [a[0] + ((b[0] - a[0]) / 2.0), a[1]],
         [a[0] + ((c[0] - a[0]) / 2.0), a[1] - ((a[1] - c[1]) / 2.0)],
         [a[0], a[1]]],
        [[a[0] + ((b[0] - a[0]) / 2.0), a[1]],
         [b[0], b[1]],
         [b[0] - ((b[0] - c[0]) / 2.0), b[1] - ((b[1] - c[1]) / 2.0)],
         [a[0] + ((b[0] - a[0]) / 2.0), a[1]]],
        [
         [a[0] + ((c[0] - a[0]) / 2.0), a[1] - ((a[1] - c[1]) / 2.0)],
         [b[0] - ((b[0] - c[0]) / 2.0), b[1] - ((b[1] - c[1]) / 2.0)],
         [c[0], c[1]],
         [a[0] + ((c[0] - a[0]) / 2.0), a[1] - ((a[1] - c[1]) / 2.0)]]]
}

fn sierpinski(depth: i32, vertices: [[f64; 2]; 4]) -> Vec<[[f64; 2]; 4]> {
    let mut triangles = vec![];

    match depth {
        0 => triangles,
        d @ _ => {
            let sub_tris = sub_triangles(vertices);

            triangles.push(sub_tris[0]);

            for sub_tri in &sub_tris[1..] {
                for t in sierpinski(d - 1, *sub_tri) {
                    triangles.push(t);
                }
            }

            triangles
        },
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

    let triangles = sierpinski(8, [[50.0, 750.0], [950.0, 750.0],
                                   [500.0, 50.0], [50.0, 750.0]]);

    let mut app = App{gl: GlGraphics::new(opengl), triangles: triangles};

    for e in window.events().max_fps(1) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
    }
}
