mod controller;
mod fractal;
mod renderer;
use controller::{Controller, Direction};
use sdl2::pixels::Color;

#[tokio::main]
async fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut renderer = renderer::Renderer::new(&sdl_context);
    let mut controller = Controller::new(&sdl_context);

    let mut center_x = 0.0;
    let mut center_y = 0.0;
    const MOVE_SPEED: f64 = 0.05;

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

        let fractal_data = fractal::compute_fractal(
            center_x,
            center_y,
            renderer::WIDTH as usize,
            renderer::HEIGHT as usize,
        )
        .await;

        for (y, row) in fractal_data.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                let color = (val as f64 * 255.0 / fractal::MAX_ITER as f64) as u8;
                let color = Color::RGB(color, color, color);
                renderer.draw_point(x as i32, y as i32, color)
            }
        }
        renderer.canvas.present();
    }
}
