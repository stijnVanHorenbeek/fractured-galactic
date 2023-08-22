use sdl2::pixels::Color;

extern crate sdl2;

pub(crate) const WIDTH: u32 = 1024;
pub(crate) const HEIGHT: u32 = 512;

pub struct Renderer {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Renderer {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Imagination Land", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        Renderer { canvas }
    }

    pub fn draw_point(&mut self, x: i32, y: i32, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.draw_point((x, y)).unwrap();
    }

    pub fn draw_crosshair(&mut self, x: i32, y: i32, color: Color, length: i32) {
        self.canvas.set_draw_color(color);
        // horizontal line
        self.canvas
            .draw_line((x - length, y), (x + length, y))
            .unwrap();
        // vertical line
        self.canvas
            .draw_line((x, y - length), (x, y + length))
            .unwrap();
    }
}
