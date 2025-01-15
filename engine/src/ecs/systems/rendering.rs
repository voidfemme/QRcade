use crate::engine::rendering::Sdl2Renderer;
use crate::lua::runtime::state_manager::StateManager;
use std::rc::Rc;

pub fn render_system(
    state_manager: Rc<StateManager>,
    renderer: &mut Sdl2Renderer,
    scale: f32,
    debug: bool,
) {
    // Try to borrow the state for reading
    if let Ok(state) = state_manager.state.try_borrow() {
        // First render tilemaps (they should be in the background)
        for (&entity, tilemap) in &state.tilemaps {
            // Get tilemap dimensions
            let tile_size = tilemap.tile_size;
            
            // Render each tile
            for y in 0..tilemap.height {
                for x in 0..tilemap.width {
                    if let Some(tile) = tilemap.get_tile(x, y) {
                        // Create a square asset for each tile
                        if let Some(asset) = state_manager.get_asset("rectangle") {
                            let x_pos = (x * tile_size) as i32;
                            let y_pos = (y * tile_size) as i32;
                            
                            // Render the tile using the square asset with explicit size
                            let shape_data = Some(crate::ecs::components::sprite::SpriteShapeData::Rectangle {
                                width: tile_size as f32,
                                height: tile_size as f32,
                            });
                            
                            state_manager.render_asset(
                                asset,
                                shape_data.as_ref(),
                                x_pos,
                                y_pos,
                                tile.color,
                                renderer,
                                scale,
                                debug,
                            );
                        }
                    }
                }
            }
        }

        // Then render sprites (they should be on top of tilemaps)
        for (&entity, transform) in &state.transforms {
            if let Some(sprite) = state.sprites.get(&entity) {

                // Get the asset definition from the state manager
                if let Some(asset) = state_manager.get_asset(&sprite.asset_name) {
                    // Let the asset system handle the rendering based on the shape type
                    state_manager.render_asset(
                        asset,
                        sprite.shape_data.as_ref(),
                        transform.x as i32,
                        transform.y as i32,
                        sprite.color,
                        renderer,
                        scale,
                        debug,
                    );
                }
            } else {
                println!("Warning: Entity {} has transform but no sprite", entity);
            }
        }
    }
}
