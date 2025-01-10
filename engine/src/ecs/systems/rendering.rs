use crate::engine::rendering::{Renderer, Sdl2Renderer};
use crate::lua::runtime::state_manager::StateManager;
// use sdl2::pixels::Color;
// use sdl2::rect::Rect;
// use sdl2::render::Canvas;
// use sdl2::video::Window;
// use sdl2::Sdl;
use std::rc::Rc;

pub fn render_system(state_manager: Rc<StateManager>, renderer: &mut Sdl2Renderer, scale: f32) {
    // Try to borrow the state for reading
    if let Ok(state) = state_manager.state.try_borrow() {
        for (&entity, transform) in &state.transforms {
            if let Some(sprite) = state.sprites.get(&entity) {
                // Use actual sprite dimensions and color
                renderer.draw_scaled_rect(
                    transform.x as i32,
                    transform.y as i32,
                    sprite.width as u32,
                    sprite.height as u32,
                    scale,
                    sprite.color,
                    true, // Enable debug visualization
                );
            } else {
                println!("Warning: Entity {} has transform but no sprite", entity);
            }
        }
    }
}
