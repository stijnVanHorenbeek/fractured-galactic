mod controller;
mod fractal;
mod renderer;
use controller::{Controller, Direction};
use num_complex::Complex;
use sdl2::pixels::Color;

#[tokio::main]
async fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut renderer = renderer::Renderer::new(&sdl_context);
    let mut controller = Controller::new(&sdl_context);

    let mut center_x = 0.0;
    let mut center_y = 0.0;

    const MOVE_SPEED: f64 = 0.05;
    const CROSSHAIR_LENGTH: i32 = 10;
    const CROSSHAIR_COLOR: Color = Color::RGB(255, 0, 0);
    const BOUND_X_MIN: f64 = -1.8;
    const BOUND_X_MAX: f64 = 1.8;
    const BOUND_Y_MIN: f64 = -1.8;
    const BOUND_Y_MAX: f64 = 1.8;
    loop {
        let (running, direction) = controller.process_events();
        if !running {
            break;
        }

        if let Some(dir) = direction {
            match dir {
                Direction::Up => center_y -= MOVE_SPEED,
                Direction::Down => center_y += MOVE_SPEED,
                Direction::Left => center_x -= MOVE_SPEED,
                Direction::Right => center_x += MOVE_SPEED,
            }
        }
        center_x = center_x.clamp(BOUND_X_MIN, BOUND_X_MAX);
        center_y = center_y.clamp(BOUND_Y_MIN, BOUND_Y_MAX);

        let mandelbrot_data = fractal::compute_mandelbrot(
            center_x,
            center_y,
            (renderer::WIDTH / 2) as usize,
            renderer::HEIGHT as usize,
        )
        .await;

        let julia_data = fractal::compute_julia_set(
            Complex::new(center_x, center_y),
            0.0,
            0.0,
            (renderer::WIDTH / 2) as usize,
            renderer::HEIGHT as usize,
        )
        .await;

        for (y, row) in mandelbrot_data.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                let color = (val as f64 * 255.0 / fractal::MAX_ITER as f64) as u8;
                let color = Color::RGB(color, color, color);
                renderer.draw_point(x as i32, y as i32, color)
            }
        }

        for (y, row) in julia_data.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                let color = (val as f64 * 255.0 / fractal::MAX_ITER as f64) as u8;
                let color = Color::RGB(color, color, color);
                renderer.draw_point(x as i32 + renderer::WIDTH as i32 / 2, y as i32, color)
            }
        }

        let crosshair_x = (center_x + 2.0) * (renderer::WIDTH as f64 / 2.0) / 4.0;
        let crosshair_y = (center_y + 2.0) * (renderer::HEIGHT as f64) / 4.0;
        renderer.draw_crosshair(
            crosshair_x as i32,
            crosshair_y as i32,
            CROSSHAIR_COLOR,
            CROSSHAIR_LENGTH,
        );

        renderer.canvas.present();
    }
}
