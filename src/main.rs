mod controller;
mod fractal;
mod renderer;

use controller::{Controller, Direction};
use num_complex::Complex;
use renderer::Renderer;
use sdl2::pixels::Color;

const ACCELERATION: f64 = 0.01;
const DECELERATION: f64 = 0.005;
const CROSSHAIR_LENGTH: i32 = 10;
const CROSSHAIR_COLOR: Color = Color::RGB(255, 0, 0);
const BOUND_X_MIN: f64 = -1.8;
const BOUND_X_MAX: f64 = 1.8;
const BOUND_Y_MIN: f64 = -1.8;
const BOUND_Y_MAX: f64 = 1.8;

fn update_position(
    center_x: &mut f64,
    center_y: &mut f64,
    direction: Option<Direction>,
    move_speed: &mut f64,
) {
    if let Some(dir) = direction {
        *move_speed += ACCELERATION;
        *move_speed = move_speed.clamp(0.0, 1.0);
        match dir {
            Direction::Down => *center_y += *move_speed,
            Direction::Up => *center_y -= *move_speed,
            Direction::Left => *center_x -= *move_speed,
            Direction::Right => *center_x += *move_speed,
        }
    } else {
        *move_speed -= DECELERATION;
        *move_speed = move_speed.max(0.0);
    }

    *center_x = center_x.clamp(BOUND_X_MIN, BOUND_X_MAX);
    *center_y = center_y.clamp(BOUND_Y_MIN, BOUND_Y_MAX);
}

async fn calc_fractals(
    center_x: f64,
    center_y: f64,
    width: usize,
    height: usize,
) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mandelbrot_data = fractal::compute_mandelbrot(
        center_x,
        center_y,
        (renderer::WIDTH / 2) as usize,
        renderer::HEIGHT as usize,
    )
    .await;

    let julia_data =
        fractal::compute_julia_set(Complex::new(center_x, center_y), 0.0, 0.0, width, height).await;

    (mandelbrot_data, julia_data)
}

fn draw_fractals(renderer: &mut Renderer, mandelbrot_data: &[Vec<u32>], julia_data: &[Vec<u32>]) {
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
}

fn draw_crosshair(renderer: &mut Renderer, center_x: f64, center_y: f64) {
    let crosshair_x = (center_x + 2.0) * (renderer::WIDTH as f64 / 2.0) / 4.0;
    let crosshair_y = (center_y + 2.0) * (renderer::HEIGHT as f64) / 4.0;
    renderer.draw_crosshair(
        crosshair_x as i32,
        crosshair_y as i32,
        CROSSHAIR_COLOR,
        CROSSHAIR_LENGTH,
    );
}

#[tokio::main]
async fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut renderer = renderer::Renderer::new(&sdl_context);
    let mut controller = Controller::new(&sdl_context);

    let mut center_x = 0.0;
    let mut center_y = 0.0;

    loop {
        let (running, direction) = controller.process_events();
        if !running {
            break;
        }

        let mut move_speed: f64 = 0.0;
        update_position(&mut center_x, &mut center_y, direction, &mut move_speed);
        let (mandelbrot, julia) = calc_fractals(
            center_x,
            center_y,
            (renderer::WIDTH / 2) as usize,
            renderer::HEIGHT as usize,
        )
        .await;

        draw_fractals(&mut renderer, &mandelbrot, &julia);

        draw_crosshair(&mut renderer, center_x, center_y);

        renderer.canvas.present();
    }
}
