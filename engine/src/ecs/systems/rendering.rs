use crate::engine::rendering::{Renderer, Sdl2Renderer};
use crate::lua::runtime::state_manager::StateManager;
use std::rc::Rc;

pub fn render_system(state_manager: Rc<StateManager>, renderer: &mut Sdl2Renderer, scale: f32) {
    // Try to borrow the state for reading
    if let Ok(state) = state_manager.state.try_borrow() {
        for (&entity, transform) in &state.transforms {
            if let Some(sprite) = state.sprites.get(&entity) {
                println!(
                    "Rendering entity {} with asset '{}'",
                    entity, sprite.asset_name
                );

                // Get the asset definition from the state manager
                if let Some(asset) = state_manager.get_asset(&sprite.asset_name) {
                    println!("Found asset, shape type: {:?}", asset.shape);

                    // Let the asset system handle the rendering based on the shape type
                    state_manager.render_asset(
                        asset,
                        transform.x as i32,
                        transform.y as i32,
                        sprite.color,
                        renderer,
                        scale,
                    );
                }
            } else {
                println!("Warning: Entity {} has transform but no sprite", entity);
            }
        }
    }
}
