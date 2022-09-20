extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::{thread, time};

use crate::complex;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const W: f64 = WIDTH as f64 / 2.0;
const H: f64 = HEIGHT as f64 / 2.0;

const ESCAPE_RADIUS: f64 = 3.5;
const MAX_ITER: usize = 100;

pub struct App<'a> {
    gl: GlGraphics,                  // OpenGL drawing backend.
    julia_set: &'a complex::Complex, // Julia set
}

impl App<'_> {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [0.1, 0.1, 0.1, 1.0];

        let square = rectangle::square(0.0, 0.0, 1.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let a: f64 = (x as f64 - W) / W as f64 * 3.0;
                    let b: f64 = (y as f64 - H) / H as f64 * 3.0;
                    let mut cplx = complex::Complex::new(complex::Form::Cartesian(a, b));

                    let col: f32 = converges(&mut cplx, self.julia_set) as f32 / MAX_ITER as f32;
                    let color: [f32; 4] = [col, col, col, 1.0];
                    let transform = c.transform.trans(x as f64, y as f64);
                    // Draw a box rotating around the middle of the screen.
                    rectangle(color, square, transform, gl);
                }
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
    }
}

pub fn draw(julia_set: &complex::Complex) {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Julia set", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        julia_set,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

fn converges(c: &mut complex::Complex, julia_set: &complex::Complex) -> usize {
    let mut iter: usize = 0;
    let r1: f64 = c.get_r();
    // Julia set formula : Z²+C
    while c.get_r() - r1 < ESCAPE_RADIUS && iter < MAX_ITER {
        iter += 1;
        c.square();
        c.add(julia_set);
    }
    iter
}

fn _display_julia_set(julia_set: &complex::Complex) {
    print!(
        "\nJulia set C = {} + i{}\n\n",
        julia_set.get_a(),
        julia_set.get_b()
    );
    let width: isize = 100;
    let height: isize = 35;
    for y in -height..height {
        for x in -width..width {
            let a: f64 = x as f64 / width as f64 * 0.9;
            let b: f64 = y as f64 / height as f64 * 1.6;
            let mut _c = complex::Complex::new(complex::Form::Cartesian(a, b));
        }
        print!("\n");
    }
}
