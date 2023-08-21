extern crate num_complex;
extern crate sdl2;

use num_complex::Complex;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const MAX_ITER: i32 = 100;

fn mandelbrot(c: Complex<f64>) -> i32 {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..MAX_ITER {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    MAX_ITER
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Mandelbrot Set", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut zoom_factor = 1.0;
    let center_x = 0.0;
    let center_y = 0.0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::J),
                    ..
                } => {
                    zoom_factor *= 1.1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::K),
                    ..
                } => {
                    zoom_factor /= 1.1;
                }
                _ => {}
            }
        }

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let scale = 4.0 / zoom_factor;
                let cx = ((x as f64 / WIDTH as f64) * scale - scale / 2.0) + center_x;
                let cy = ((y as f64 / HEIGHT as f64) * scale - scale / 2.0) + center_y;
                let c = Complex::new(cx, cy);

                let val = mandelbrot(c);
                let color = (val * 255 / MAX_ITER) as u8;
                canvas.set_draw_color(Color::RGB(color, color, color));
                canvas.draw_point((x as i32, y as i32)).unwrap();
            }
        }
        canvas.present();
    }
}
