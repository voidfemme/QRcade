use std::cell::RefCell;
use std::rc::Rc;
use crate::GameState;
use crate::assets::asset_manager::{AssetManager, PrimitiveShape};
use super::Manager;

#[derive(Debug)]
pub struct EntityManager {
    state: Rc<RefCell<GameState>>,
}

impl Manager for EntityManager {
    fn new(state: Rc<RefCell<GameState>>) -> Self {
        Self { state }
    }

    fn get_state(&self) -> &Rc<RefCell<GameState>> {
        &self.state
    }
}

impl EntityManager {
    pub fn create_entity(&self) -> Result<u32, &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => Ok(state.create_entity()),
            Err(_) => Err("Failed to borrow game state")
        }
    }

    pub fn destroy_entity(&self, entity_id: u32) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                state.destroy_entity(entity_id);
                Ok(())
            }
            Err(_) => Err("Failed to borrow game state")
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
        assets: &AssetManager,
    ) -> Result<(), &'static str> {
        let asset = assets.get_by_name(asset_name).ok_or("Asset not found")?;

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
            PrimitiveShape::Triangle { x1, y1, x2, y2, x3, y3 } => {
                if let Some(params) = params {
                    let nx1: f32 = params.get("x1").unwrap_or(*x1);
                    let ny1: f32 = params.get("y1").unwrap_or(*y1);
                    let nx2: f32 = params.get("x2").unwrap_or(*x2);
                    let ny2: f32 = params.get("y2").unwrap_or(*y2);
                    let nx3: f32 = params.get("x3").unwrap_or(*x3);
                    let ny3: f32 = params.get("y3").unwrap_or(*y3);
                    crate::ecs::components::sprite::Sprite::new_triangle(
                        (r, g, b), nx1, ny1, nx2, ny2, nx3, ny3,
                    )
                } else {
                    crate::ecs::components::sprite::Sprite::new_triangle(
                        (r, g, b), *x1, *y1, *x2, *y2, *x3, *y3,
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
            Err(_) => Err("Failed to borrow game state")
        }
    }

    pub fn debug_print_entities(&self, assets: &AssetManager) -> Result<(), &'static str> {
        if let Ok(state) = self.state.try_borrow() {
            println!("\nEntity Debug Info:");
            for entity in &state.entities {
                println!("Entity {}: ", entity);

                // Print transform information
                if let Some(transform) = state.transforms.get(entity) {
                    println!("  Transform: ({}, {})", transform.x, transform.y);
                }

                // Print sprite information
                if let Some(sprite) = state.sprites.get(entity) {
                    if let Some(asset) = assets.get_by_name(&sprite.asset_name) {
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
                            PrimitiveShape::Triangle { x1, y1, x2, y2, x3, y3 } => {
                                println!(
                                    "  Sprite: Triangle ({}, {}), ({}, {}), ({}, {}) RGB({}, {}, {})",
                                    x1, y1, x2, y2, x3, y3, 
                                    sprite.color.0, sprite.color.1, sprite.color.2
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
