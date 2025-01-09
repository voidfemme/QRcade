use crate::GameState;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub fn draw_scaled_rect(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    scale: f32,
) {
    // Scale the width and height
    let scaled_width = (width as f32 * scale).round() as u32;
    let scaled_height = (height as f32 * scale).round() as u32;

    // Adjust x and y to keep the rectangle centered
    let scaled_x = x - ((scaled_width as i32 - width as i32) / 2);
    let scaled_y = y - ((scaled_height as i32 - height as i32) / 2);

    // Create the rectangle
    let rect = Rect::new(scaled_x, scaled_y, scaled_width, scaled_height);

    // Set the color and draw the rectangle
    canvas.set_draw_color(sdl2::pixels::Color::RED);
    canvas.fill_rect(rect).unwrap();
}

pub fn render_system(
    game_state: &GameState,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    scale: f32,
) {
    canvas.set_draw_color(sdl2::pixels::Color::BLACK);
    canvas.clear();

    for (&entity, transform) in &game_state.transforms {
        // Example: Draw a rectangle for each entity with a transform
        let sprite = game_state.sprites.get(&entity);
        let rect_width = 32;
        let rect_height = 32;

        // Draw the rectangle at the transformed position with scaling
        draw_scaled_rect(
            canvas,
            transform.x as i32,
            transform.y as i32,
            rect_width,
            rect_height,
            scale,
        );
    }
}

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

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }

    pub fn draw(&mut self, texture_id: String, x: f32, y: f32, scale: f32) {
        self.canvas.set_draw_color(Color::RED);
        let rect = Rect::new(x as i32, y as i32, 32, 32);
        let _ = self.canvas.fill_rect(rect);
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
