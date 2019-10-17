mod draw2d;
mod framebuffer;
mod util;

use draw2d::{Triangle2D, Vec2D};
use framebuffer::Framebuffer;
use minifb::{Window, WindowOptions};
use std::thread::sleep;
use std::time::{Duration, Instant};

const STEP_PERIOD: Duration = Duration::from_millis(1000 / 60);

macro_rules! error {
    ($($arg:tt)*) => ({
        eprint!("error: ");
        eprintln!($($arg)*);
        std::process::exit(1);
    })
}

fn next_step(window: &mut Window, fb: &mut Framebuffer) {
    window.update_with_buffer(fb.slice()).unwrap_or_else(|e| {
        error!("Could not update window buffer\n{}", e);
    });
}

fn main() {
    let mut fb = Framebuffer::new(800, 600);
    fb.fill_triangle(
        &Triangle2D::new((
            Vec2D::new(50, 50),
            Vec2D::new(100, 400),
            Vec2D::new(500, 500),
        )),
        0xFF00_7733,
    );
    fb.draw_triangle(
        &Triangle2D::new((
            Vec2D::new(50, 50),
            Vec2D::new(100, 400),
            Vec2D::new(500, 500),
        )),
        0xFFFF_FFFF,
    );
    let mut window = Window::new("view3d", fb.width, fb.height, WindowOptions::default())
        .unwrap_or_else(|e| {
            error!("Could not create window\n{}", e);
        });
    while window.is_open() {
        let begin = Instant::now();
        next_step(&mut window, &mut fb);
        let elapsed = begin.elapsed();
        if elapsed < STEP_PERIOD {
            sleep(STEP_PERIOD - elapsed);
        }
    }
}
