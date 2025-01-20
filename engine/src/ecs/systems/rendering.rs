use crate::engine::managers::state_manager::StateManager;
use crate::engine::rendering::{Renderer, Sdl2Renderer};
use sdl2::pixels::Color;
use std::rc::Rc;
use tracing::warn;

pub fn render_system(
    state_manager: Rc<StateManager>,
    renderer: &mut Sdl2Renderer,
    scale: f32,
    debug: bool,
) {
    // Try to borrow the state for reading
    if let Ok(state) = state_manager.state.try_borrow() {
        // First render tilemaps (they should be in the background)
        for (&_, tilemap) in &state.tilemaps {
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
                            let shape_data =
                                Some(crate::ecs::components::sprite::SpriteShapeData::Rectangle {
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
            } else if let Some(text) = state.texts.get(&entity) {
                // Only show warning if entity has neither sprite nor text
                if !text.visible {
                    continue;
                }

                let color = Color::RGB(text.color.0, text.color.1, text.color.2);
                let content = text.get_string();

                renderer.draw_text(
                    content,
                    transform.x as i32,
                    transform.y as i32,
                    color,
                    text.scale,
                );

                if debug {
                    // draw debug info for text entities
                    renderer.draw_bounding_box(
                        transform.x as i32,
                        transform.y as i32,
                        20, // TODO: Placeholder size, calculate actual text bounds later
                        20,
                        Color::RGB(255, 255, 255),
                    );
                }
            } else {
                // Only show warning if entity has neither sprite nor text
                if !state.texts.contains_key(&entity) {
                    warn!(
                        "Warning: Entity {} has transform but no sprite or text",
                        entity
                    );
                }
            }
        }
    }
}
