pub mod sdl;

use sdl2::pixels::Color;
// use sdl2::rect::Rect;
// use sdl2::render::Canvas;
// use sdl2::video::Window;
// use sdl2::Sdl;

// A general trait for any rendering implementation
pub trait Renderer {
    fn clear(&mut self);
    fn present(&mut self);
    fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color);
    fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: u32, color: Color);
    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: Color);
    fn draw_bounding_box(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color);
    fn draw_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color);
    fn draw_text(&mut self, text: String, x: i32, y: i32, color: Color, scale: f32);
}

// Re-export SDL renderer implementation
pub use sdl::Sdl2Renderer;
