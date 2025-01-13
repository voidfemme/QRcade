use crate::assets::asset_manager::{AssetManager, BuiltInAsset, PrimitiveShape};
use crate::ecs::components::component::GameState;
use crate::ecs::components::sprite::SpriteShapeData;
use crate::ecs::components::tilemap::{Tile, Tilemap, TilemapQuery, TilemapQueryResult};
use crate::ecs::components::velocity::Velocity;
use crate::ecs::systems::collision_system::CollisionSystem;
use crate::ecs::systems::input_system::InputSystem;
use crate::{Renderer, Sdl2Renderer};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::cell::RefCell;
use std::rc::Rc;

pub struct StateManager {
    pub state: Rc<RefCell<GameState>>,
    input: RefCell<InputSystem>,
    assets: AssetManager,
}

impl StateManager {
    pub fn new(state: Rc<RefCell<GameState>>) -> Self {
        Self {
            state,
            input: RefCell::new(InputSystem::new()),
            assets: AssetManager::new(),
        }
    }

    pub fn check_position_walkable(
        &self,
        entity_id: u32,
        tilemap_id: u32,
        x: f32,
        y: f32,
    ) -> Result<bool, &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                // First check if the position is in a walkable tile
                if let Some(tilemap) = state.tilemaps.get(&tilemap_id) {
                    // Check for collisions using the collision system
                    match CollisionSystem::check_entity_tilemap_collision(
                        &state,
                        &self.assets,
                        entity_id,
                        tilemap_id,
                        x,
                        y,
                    ) {
                        Ok(collision) => Ok(!collision), // If there's no collision, it's walkable
                        Err(e) => Err(e),
                    }
                } else {
                    Err("Tilemap not found")
                }
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn create_tilemap(
        &self,
        entity_id: u32,
        width: u32,
        height: u32,
        tile_size: u32,
    ) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                // Check if entity exists
                if !state.entities.contains(&entity_id) {
                    return Err("Entity does not exist");
                }

                // Create new tilemap
                let tilemap = Tilemap::new(width, height, tile_size);

                // Add tilemap to state
                state.tilemaps.insert(entity_id, tilemap);
                Ok(())
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn query_tilemap(
        &self,
        entity_id: u32,
        query: TilemapQuery,
    ) -> Result<TilemapQueryResult, &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                let tilemap = state
                    .tilemaps
                    .get(&entity_id)
                    .ok_or("No tilemap found for entity")?;
                tilemap.query(query)
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn set_tile(
        &self,
        entity_id: u32,
        x: u32,
        y: u32,
        tile_id: u32,
        walkable: bool,
        color: (u8, u8, u8),
    ) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                // Get tilemap for entity
                let tilemap = state
                    .tilemaps
                    .get_mut(&entity_id)
                    .ok_or("No tilemap found for entity")?;

                // Create new tile
                let tile = Tile {
                    tile_id,
                    walkable,
                    color,
                };

                // Set tile in tilemap
                tilemap.set_tile(x, y, tile)
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn clear_tile(&self, entity_id: u32, x: u32, y: u32) -> Result<bool, &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                // Get tilemap for entity
                let tilemap = state
                    .tilemaps
                    .get_mut(&entity_id)
                    .ok_or("No tilemap found for entity")?;

                // Return walkable status
                Ok(tilemap.is_walkable(x, y))
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn get_tilemap(&self, entity_id: u32) -> Result<Option<Tilemap>, &'static str> {
        match self.state.try_borrow() {
            Ok(state) => Ok(state.tilemaps.get(&entity_id).cloned()),
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn is_tile_walkable(&self, entity_id: u32, x: u32, y: u32) -> Result<bool, &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                // Get tilemap for entity
                let tilemap = state
                    .tilemaps
                    .get(&entity_id)
                    .ok_or("No tilemap found for entity")?;

                // Return walkable status
                Ok(tilemap.is_walkable(x, y))
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    // Add this to debug_print_entities to show tilemap information
    pub fn set_velocity(&self, entity_id: u32, vx: f32, vy: f32) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                state.add_velocity(entity_id, Velocity::new(vx, vy));
                Ok(())
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn get_velocity(&self, entity_id: u32) -> Result<(f32, f32), &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                if let Some(velocity) = state.velocities.get(&entity_id) {
                    Ok((velocity.dx, velocity.dy))
                } else {
                    Ok((0.0, 0.0))
                }
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn create_entity(&self) -> Result<u32, &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => Ok(state.create_entity()),
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn destroy_entity(&self, entity_id: u32) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                state.destroy_entity(entity_id);
                Ok(())
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn set_transform(
        &self,
        entity_id: u32,
        x: f32,
        y: f32,
        rotation: f32,
    ) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                state.add_transform(
                    entity_id,
                    crate::ecs::components::transform::Transform::new(x, y, rotation),
                );
                Ok(())
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn add_sprite(
        &self,
        entity_id: u32,
        asset_name: &str,
        r: u8,
        g: u8,
        b: u8,
        params: Option<mlua::Table>,
    ) -> Result<(), &'static str> {
        let asset = self.get_asset(asset_name).ok_or("Asset not found")?;

        let sprite = match &asset.shape {
            PrimitiveShape::Rectangle { width, height } => {
                if let Some(params) = params {
                    let new_width: f32 = params.get("width").unwrap_or(*width);
                    let new_height: f32 = params.get("height").unwrap_or(*height);
                    crate::ecs::components::sprite::Sprite::new_rectangle(
                        new_width, new_height, r, g, b,
                    )
                } else {
                    crate::ecs::components::sprite::Sprite::new(asset_name, r, g, b, None)
                }
            }
            PrimitiveShape::Circle { radius } => {
                if let Some(params) = params {
                    let new_radius: f32 = params.get("radius").unwrap_or(*radius);
                    crate::ecs::components::sprite::Sprite::new_circle(new_radius, r, g, b)
                } else {
                    crate::ecs::components::sprite::Sprite::new(asset_name, r, g, b, None)
                }
            }
            PrimitiveShape::Triangle {
                x1,
                y1,
                x2,
                y2,
                x3,
                y3,
            } => {
                if let Some(params) = params {
                    let nx1: f32 = params.get("x1").unwrap_or(*x1);
                    let ny1: f32 = params.get("y1").unwrap_or(*y1);
                    let nx2: f32 = params.get("x2").unwrap_or(*x2);
                    let ny2: f32 = params.get("y2").unwrap_or(*y2);
                    let nx3: f32 = params.get("x3").unwrap_or(*x3);
                    let ny3: f32 = params.get("y3").unwrap_or(*y3);
                    crate::ecs::components::sprite::Sprite::new_triangle(
                        r, g, b, nx1, ny1, nx2, ny2, nx3, ny3,
                    )
                } else {
                    crate::ecs::components::sprite::Sprite::new_triangle(
                        r, g, b, *x1, *y1, *x2, *y2, *x3, *y3,
                    )
                }
            }
            PrimitiveShape::Line { x2, y2 } => {
                if let Some(params) = params {
                    let nx2: f32 = params.get("x2").unwrap_or(*x2);
                    let ny2: f32 = params.get("y2").unwrap_or(*y2);
                    crate::ecs::components::sprite::Sprite::new_line(r, g, b, nx2, ny2)
                } else {
                    crate::ecs::components::sprite::Sprite::new_line(r, g, b, *x2, *y2)
                }
            }
        };

        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                state.add_sprite(entity_id, sprite);
                Ok(())
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }
    pub fn get_transform(&self, entity_id: u32) -> Result<(f32, f32, f32), &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                if let Some(transform) = state.transforms.get(&entity_id) {
                    Ok((transform.x, transform.y, transform.rotation))
                } else {
                    Err("Entity does not have a transform component")
                }
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn get_asset(&self, name: &str) -> Option<&BuiltInAsset> {
        println!("Attempting to get asset with name: '{}'", name);
        let result = self.assets.get_by_name(name);
        println!(
            "Result: {}",
            if result.is_some() {
                "found"
            } else {
                "not found"
            }
        );
        result
    }

    pub fn handle_key(&self, keycode: Keycode, pressed: bool) -> Result<(), &'static str> {
        match self.input.try_borrow_mut() {
            Ok(mut input) => {
                if pressed {
                    input.set_key_pressed(keycode);
                } else {
                    input.set_key_released(keycode);
                }
                Ok(())
            }
            Err(_) => Err("Failed to borrow input system"),
        }
    }

    pub fn is_key_pressed(&self, keycode: Keycode) -> Result<bool, &'static str> {
        match self.input.try_borrow() {
            Ok(input) => Ok(input.is_key_pressed(keycode)),
            Err(_) => Err("Failed to borrow input system"),
        }
    }

    pub fn check_collision(&self, entity1: u32, entity2: u32) -> Result<bool, &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                CollisionSystem::are_entities_colliding(&state, &self.assets, entity1, entity2)
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn render_asset(
        &self,
        asset: &BuiltInAsset,
        shape_data: Option<&SpriteShapeData>,
        x: i32,
        y: i32,
        color: (u8, u8, u8),
        renderer: &mut Sdl2Renderer,
        scale: f32,
        debug: bool,
    ) {
        match &asset.shape {
            PrimitiveShape::Rectangle { width, height } => {
                let (final_width, final_height) = if let Some(SpriteShapeData::Rectangle {
                    width: w,
                    height: h,
                }) = shape_data
                {
                    (*w, *h)
                } else {
                    (*width, *height)
                };

                renderer.draw_rect(
                    x,
                    y,
                    (final_width * scale) as u32,
                    (final_height * scale) as u32,
                    Color::RGB(color.0, color.1, color.2),
                );
            }
            PrimitiveShape::Circle { radius } => {
                // Use custom radius if provided
                let final_radius = if let Some(SpriteShapeData::Circle { radius: r }) = shape_data {
                    *r
                } else {
                    *radius
                };
                let scaled_radius = (final_radius * scale) as u32;
                renderer.draw_circle(
                    x + scaled_radius as i32,
                    y + scaled_radius as i32,
                    scaled_radius,
                    Color::RGB(color.0, color.1, color.2),
                );

                if debug {
                    let diameter = final_radius * 2.0 * scale;
                    renderer.draw_bounding_box(
                        x + (diameter as i32 / 2),
                        y + (diameter as i32 / 2),
                        diameter as u32,
                        diameter as u32,
                        Color::RGB(255, 255, 255),
                    );
                }
            }
            PrimitiveShape::Triangle {
                x1,
                y1,
                x2,
                y2,
                x3,
                y3,
            } => {
                let color = Color::RGB(color.0, color.1, color.2);
                // Use custom vertices if provided
                let vertices = if let Some(SpriteShapeData::Triangle { vertices }) = shape_data {
                    vertices
                } else {
                    &[(*x1, *y1), (*x2, *y2), (*x3, *y3)]
                };

                let sx1 = x + (vertices[0].0 * scale) as i32;
                let sy1 = y + (vertices[0].1 * scale) as i32;
                let sx2 = x + (vertices[1].0 * scale) as i32;
                let sy2 = y + (vertices[1].1 * scale) as i32;
                let sx3 = x + (vertices[2].0 * scale) as i32;
                let sy3 = y + (vertices[2].1 * scale) as i32;

                renderer.draw_triangle(sx1, sy1, sx2, sy2, sx3, sy3, color);

                if debug {
                    let min_x = sx1.min(sx2).min(sx3);
                    let max_x = sx1.max(sx2).max(sx3);
                    let min_y = sy1.min(sy2).min(sy3);
                    let max_y = sy1.max(sy2).max(sy3);

                    renderer.draw_bounding_box(
                        (min_x + max_x) / 2,
                        (min_y + max_y) / 2,
                        (max_x - min_x) as u32,
                        (max_y - min_y) as u32,
                        Color::RGB(255, 255, 255),
                    );
                }
            }
            PrimitiveShape::Line { x2, y2 } => {
                let color = Color::RGB(color.0, color.1, color.2);
                // Use custom line endpoint if provided
                let end = if let Some(SpriteShapeData::Line { end }) = shape_data {
                    end
                } else {
                    &(*x2, *y2)
                };

                let end_x = x + (end.0 * scale) as i32;
                let end_y = y + (end.1 * scale) as i32;

                renderer.draw_line(x, y, end_x, end_y, color);

                if debug {
                    let min_x = x.min(end_x);
                    let max_x = x.max(end_x);
                    let min_y = y.min(end_y);
                    let max_y = y.max(end_y);

                    renderer.draw_bounding_box(
                        (min_x + max_x) / 2,
                        (min_y + max_y) / 2,
                        (max_x - min_x) as u32,
                        (max_y - min_y) as u32,
                        Color::RGB(255, 255, 255),
                    );
                }
            }
        }
    }

    pub fn debug_print_tilemap(&self, entity_id: u32) -> Result<(), &'static str> {
        if let Ok(state) = self.state.try_borrow() {
            if let Some(tilemap) = state.tilemaps.get(&entity_id) {
                println!("Tilemap for Entity {}:", entity_id);
                println!("  Size: {}x{}", tilemap.width, tilemap.height);
                println!("  Tile size: {}", tilemap.tile_size);
                println!("  Total tiles: {}", tilemap.tiles.len());

                let occupied_tiles = tilemap.tiles.iter().filter(|tile| tile.is_some()).count();
                println!("  Occupied tiles: {}", occupied_tiles);
            }
            Ok(())
        } else {
            Err("Failed to borrow game state")
        }
    }

    pub fn debug_print_entities(&self) -> Result<(), &'static str> {
        if let Ok(state) = self.state.try_borrow() {
            println!("\nEntity Debug Info:");
            for entity in &state.entities {
                println!("Entity {}: ", entity);

                // Print transform information
                if let Some(transform) = state.transforms.get(entity) {
                    println!("  Transform: ({}, {})", transform.x, transform.y);
                }

                // Print sprite information, now including the asset type
                if let Some(sprite) = state.sprites.get(entity) {
                    // Look up the asset to get shape information
                    if let Some(asset) = self.get_asset(&sprite.asset_name) {
                        // Print different information based on the shape type
                        match &asset.shape {
                            PrimitiveShape::Rectangle { width, height } => {
                                println!(
                                    "  Sprite: Rectangle {}x{} RGB({}, {}, {})",
                                    width, height, sprite.color.0, sprite.color.1, sprite.color.2
                                );
                            }
                            PrimitiveShape::Circle { radius } => {
                                println!(
                                    "  Sprite: Circle radius {} RGB({}, {}, {})",
                                    radius, sprite.color.0, sprite.color.1, sprite.color.2
                                );
                            }
                            PrimitiveShape::Triangle {
                                x1,
                                y1,
                                x2,
                                y2,
                                x3,
                                y3,
                            } => {
                                println!(
                                    "  Sprite: Triangle ({}, {}), ({}, {}), ({}, {}) RGB({}, {}, {})",
                                    x1, y1, x2, y2, x3, y3, sprite.color.0, sprite.color.1, sprite.color.2
                                );
                            }
                            PrimitiveShape::Line { x2, y2 } => {
                                println!(
                                    "  Sprite: Line: ({}, {}) RGB({}, {}, {})",
                                    x2, y2, sprite.color.0, sprite.color.1, sprite.color.2
                                );
                            }
                        }
                    } else {
                        println!(
                            "  Sprite: Unknown asset '{}' RGB({}, {}, {})",
                            sprite.asset_name, sprite.color.0, sprite.color.1, sprite.color.2
                        );
                    }
                }
            }
            Ok(())
        } else {
            Err("Failed to borrow game state")
        }
    }
}
