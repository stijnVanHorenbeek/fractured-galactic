mod controller;
mod fractal;
mod renderer;
use controller::Controller;
use fractal::mandelbrot;
use sdl2::pixels::Color;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut renderer = renderer::Renderer::new(&sdl_context);
    let mut controller = Controller::new(&sdl_context);

    while controller.process_events() {
        for x in 0..renderer::WIDTH {
            for y in 0..renderer::HEIGHT {
                let cx = (x as f64 / renderer::WIDTH as f64) * 4.0 - 2.0;
                let cy = (y as f64 / renderer::HEIGHT as f64) * 4.0 - 2.0;
                let c = num_complex::Complex::new(cx, cy);
                let val = mandelbrot(c);
                let color_value = (val * 255 / fractal::MAX_ITER) as u8;
                let color = Color::RGB(color_value, color_value, color_value);
                renderer.draw_point(x as i32, y as i32, color);
            }
        }
        renderer.canvas.present();
    }
}
