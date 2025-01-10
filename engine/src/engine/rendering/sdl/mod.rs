use crate::engine::rendering::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub struct Sdl2Renderer {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
}

impl Sdl2Renderer {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Sdl2Renderer {
            sdl_context,
            canvas,
        }
    }
}

impl Renderer for Sdl2Renderer {
    fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(40, 40, 40));
        self.canvas.clear();
    }

    fn present(&mut self) {
        self.canvas.present();
    }

    fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color) {
        self.canvas.set_draw_color(color);
        let rect = Rect::new(x, y, width, height);
        let _ = self.canvas.fill_rect(rect);
    }

    fn draw_scaled_rect(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        scale: f32,
        color: (u8, u8, u8),
        debug: bool,
    ) {
        // Scale the width and height
        let scaled_width = (width as f32 * scale).round() as u32;
        let scaled_height = (height as f32 * scale).round() as u32;

        // Adjust x and y to keep the rectangle centered
        let scaled_x = x - ((scaled_width as i32 - width as i32) / 2);
        let scaled_y = y - ((scaled_height as i32 - height as i32) / 2);

        // Create the rectangle
        let rect = Rect::new(scaled_x, scaled_y, scaled_width, scaled_height);

        // Draw the filled rectangle with sprite color
        self.canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
        self.canvas.fill_rect(rect).unwrap();

        // If debug is enabled, draw a white border
        if debug {
            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            self.canvas.draw_rect(rect).unwrap();

            // print debug info
            println!(
                "Drawing entity at ({}, {}) with size {}x{} and color {:?}",
                x, y, scaled_width, scaled_height, color
            )
        }
    }
}
