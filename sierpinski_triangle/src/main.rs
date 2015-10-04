extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

type Triangle = [[f64; 2]; 3];

struct App {
    gl: GlGraphics,
    triangles: Vec<Triangle>,
    index: usize,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let i = self.index;
        let triangle = self.triangles[i];

        self.gl.draw(args.viewport(), |c, gl| {
            if i == 0 {
                clear(WHITE, gl);
            }

            let transform = c.transform.trans(0.0, 0.0);

            polygon(BLACK, &triangle, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        if (self.index as i32) + 1 >= self.triangles.len() as i32 {
            self.index = 0
        } else {
            self.index += 1;
        }
    }
}

fn sub_triangles(v: &Triangle) -> [Triangle; 4] {
    let (a, b, c) = (v[0], v[1], v[2]); // Vector destructuring is not yet stable in Rust 1.3.0.
    let (sax, say, sbx, sby, scx, scy) = (
            a[0] + ((b[0] - a[0]) / 2.0),
            a[1],
            b[0] - ((b[0] - c[0]) / 2.0),
            b[1] - ((b[1] - c[1]) / 2.0),
            a[0] + ((c[0] - a[0]) / 2.0),
            a[1] - ((a[1] - c[1]) / 2.0));

    [[[sax, say], [sbx, sby], [scx, scy]],
     [[a[0], a[1]], [sax, say], [scx, scy]],
     [[sax, say], [b[0], b[1]], [sbx, sby]],
     [[scx, scy], [sbx, sby], [c[0], c[1]]]]
}

fn sierpinski(depth: i32, vertices: Triangle) -> Vec<Triangle> {
    let mut triangles = vec![];

    match depth {
        0 => triangles,
        d => {
            let sub_tris = sub_triangles(&vertices);

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
    const FPS: u64 = 60;
    let opengl = OpenGL::V3_2;
    let window: Window = WindowSettings::new(
        "Sierpinski-Triangle",
        [1000, 800])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let triangles = sierpinski(8, [[50.0, 750.0], [950.0, 750.0], [500.0, 50.0]]);
    let mut app = App{gl: GlGraphics::new(opengl), triangles: triangles, index: 0};

    for e in window.events().max_fps(FPS).ups(FPS) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

    }
}
