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
}

// Re-export SDL renderer implementation
pub use sdl::Sdl2Renderer;
