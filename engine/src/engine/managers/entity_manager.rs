use super::Manager;
use crate::assets::asset_manager::{AssetManager, PrimitiveShape};
use crate::GameState;
use std::cell::RefCell;
use std::rc::Rc;
use tracing::{debug, debug_span, error, info, warn};

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

    pub fn add_sprite(
        &self,
        entity_id: u32,
        asset_name: &str,
        color: (u8, u8, u8),
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
                        new_width, new_height, color.0, color.1, color.2,
                    )
                } else {
                    crate::ecs::components::sprite::Sprite::new(
                        asset_name, color.0, color.1, color.2, None,
                    )
                }
            }
            PrimitiveShape::Circle { radius } => {
                if let Some(params) = params {
                    let new_radius: f32 = params.get("radius").unwrap_or(*radius);
                    crate::ecs::components::sprite::Sprite::new_circle(
                        new_radius, color.0, color.1, color.2,
                    )
                } else {
                    crate::ecs::components::sprite::Sprite::new(
                        asset_name, color.0, color.1, color.2, None,
                    )
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
                        (color.0, color.1, color.2),
                        nx1,
                        ny1,
                        nx2,
                        ny2,
                        nx3,
                        ny3,
                    )
                } else {
                    crate::ecs::components::sprite::Sprite::new_triangle(
                        (color.0, color.1, color.2),
                        *x1,
                        *y1,
                        *x2,
                        *y2,
                        *x3,
                        *y3,
                    )
                }
            }
            PrimitiveShape::Line { x2, y2 } => {
                if let Some(params) = params {
                    let nx2: f32 = params.get("x2").unwrap_or(*x2);
                    let ny2: f32 = params.get("y2").unwrap_or(*y2);
                    crate::ecs::components::sprite::Sprite::new_line(
                        color.0, color.1, color.2, nx2, ny2,
                    )
                } else {
                    crate::ecs::components::sprite::Sprite::new_line(
                        color.0, color.1, color.2, *x2, *y2,
                    )
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

    pub fn debug_print_entities(&self, assets: &AssetManager) -> Result<(), &'static str> {
        if let Ok(state) = self.state.try_borrow() {
            info!("Starting entity debug dump");

            for entity in &state.entities {
                let entity_span = debug_span!("entity", id = ?entity).entered();

                // Print transform information
                if let Some(transform) = state.transforms.get(entity) {
                    debug!(x = transform.x, y = transform.y, "Transform position");
                }

                // Print sprite information
                if let Some(sprite) = state.sprites.get(entity) {
                    if let Some(asset) = assets.get_by_name(&sprite.asset_name) {
                        match &asset.shape {
                            PrimitiveShape::Rectangle { width, height } => {
                                debug!(
                                    shape = "rectangle",
                                    width = width,
                                    height = height,
                                    r = sprite.color.0,
                                    g = sprite.color.1,
                                    b = sprite.color.2,
                                    "Sprite details"
                                );
                            }
                            PrimitiveShape::Circle { radius } => {
                                debug!(
                                    shape = "circle",
                                    radius = radius,
                                    r = sprite.color.0,
                                    g = sprite.color.1,
                                    b = sprite.color.2,
                                    "Sprite details"
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
                                debug!(
                                    shape = "triangle",
                                    point1 = ?(*x1, *y1),
                                    point2 = ?(*x2, *y2),
                                    point3 = ?(*x3, *y3),
                                    r = sprite.color.0,
                                    g = sprite.color.1,
                                    b = sprite.color.2,
                                    "Sprite details"
                                );
                            }
                            PrimitiveShape::Line { x2, y2 } => {
                                debug!(
                                    shape = "line",
                                    end_point = ?(*x2, *y2),
                                    r = sprite.color.0,
                                    g = sprite.color.1,
                                    b = sprite.color.2,
                                    "Sprite details"
                                );
                            }
                        }
                    } else {
                        warn!(
                            asset_name = sprite.asset_name,
                            r = sprite.color.0,
                            g = sprite.color.1,
                            b = sprite.color.2,
                            "Unknown asset for sprite"
                        );
                    }
                }

                drop(entity_span);
            }

            debug!("Entity debug dump complete");
            Ok(())
        } else {
            error!("Failed to borrow game state for debug print");
            Err("Failed to borrow game state")
        }
    }
}
