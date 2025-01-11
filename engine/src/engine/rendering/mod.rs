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
    fn draw_scaled_rect(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        scale: f32,
        color: (u8, u8, u8),
        debug: bool,
    );
    fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: u32, color: Color);
    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: Color);
    fn draw_bounding_box(&mut self, x: i32, y: i32, width: u32, height: u32, color: Color);
}

// Re-export SDL renderer implementation
pub use sdl::Sdl2Renderer;
