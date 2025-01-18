use super::Manager;
use crate::assets::asset_manager::{AssetManager, PrimitiveShape};
use crate::ecs::components::component::GameState;
use std::cell::RefCell;
use std::rc::Rc;
use tracing::debug;

// Keep track of dragging state
#[derive(Debug)]
struct DraggingState {
    entity_id: u32,
    offset_x: f32,
    offset_y: f32,
}

#[derive(Debug)]
pub struct DragDropManager {
    state: Rc<RefCell<GameState>>,
    assets: Rc<AssetManager>,
    dragging_state: RefCell<Option<DraggingState>>,
}

impl Manager for DragDropManager {
    fn new(state: Rc<RefCell<GameState>>) -> Self {
        // This implementation is just to satisfy the trait.
        // We'll use new_with_assets instead.
        Self {
            state,
            assets: Rc::new(AssetManager::new()),
            dragging_state: RefCell::new(None),
        }
    }

    fn get_state(&self) -> &Rc<RefCell<GameState>> {
        &self.state
    }
}

impl DragDropManager {
    // pub fn new_with_assets(state: Rc<RefCell<GameState>>, assets: Rc<AssetManager>) -> Self {
    //     Self {
    //         state,
    //         assets,
    //         dragging_state: RefCell::new(None),
    //     }
    // }

    pub fn get_entity_at_point(&self, x: f32, y: f32) -> Result<Option<u32>, &'static str> {
        let state = self
            .state
            .try_borrow()
            .map_err(|_| "Failed to borrow game state")?;

        debug!("Checking for entity at point: ({}, {})", x, y);

        for (&entity_id, transform) in &state.transforms {
            if let Some(sprite) = state.sprites.get(&entity_id) {
                if let Some(asset) = self.assets.get_by_name(&sprite.asset_name) {
                    match &asset.shape {
                        PrimitiveShape::Circle { radius } => {
                            let circle_center_x = transform.x + radius;
                            let circle_center_y = transform.y + radius;

                            let dx = x - circle_center_x;
                            let dy = y - circle_center_y;
                            let dist_squared = dx * dx + dy * dy;

                            let interaction_radius = radius;
                            let interaction_radius_squared =
                                interaction_radius * interaction_radius;

                            debug!(
                                "Circle at ({}, {}), center at ({}, {}), radius {}",
                                transform.x, transform.y, circle_center_x, circle_center_y, radius
                            );
                            debug!(
                                "Distance squared from center: {}, interaction radius squared: {}",
                                dist_squared, interaction_radius_squared
                            );

                            if dist_squared <= interaction_radius_squared {
                                debug!("Found entity {} under mouse", entity_id);
                                return Ok(Some(entity_id));
                            }
                        }
                        PrimitiveShape::Rectangle { width, height } => {
                            let rect_left = transform.x;
                            let rect_right = transform.x + width;
                            let rect_top = transform.y;
                            let rect_bottom = transform.y + height;

                            if x >= rect_left
                                && x <= rect_right
                                && y >= rect_top
                                && y <= rect_bottom
                            {
                                debug!("Found rectangle entity {} under mouse", entity_id);
                                return Ok(Some(entity_id));
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
                            let world_x1 = transform.x + x1;
                            let world_y1 = transform.y + y1;
                            let world_x2 = transform.x + x2;
                            let world_y2 = transform.y + y2;
                            let world_x3 = transform.x + x3;
                            let world_y3 = transform.y + y3;

                            let area = 0.5
                                * (-world_y2 * world_x3
                                    + world_y1 * (-world_x2 + world_x3)
                                    + world_x1 * (world_y2 - world_y3)
                                    + world_x2 * world_y3);

                            let s = 1.0 / (2.0 * area)
                                * (world_y1 * world_x3 - world_x1 * world_y3
                                    + (world_y3 - world_y1) * x
                                    + (world_x1 - world_x3) * y);

                            let t = 1.0 / (2.0 * area)
                                * (world_x1 * world_y2 - world_y1 * world_x2
                                    + (world_y1 - world_y2) * x
                                    + (world_x2 - world_x1) * y);

                            if s > 0.0 && t > 0.0 && 1.0 - s - t > 0.0 {
                                debug!("Found triangle entity {} under mouse", entity_id);
                                return Ok(Some(entity_id));
                            }
                        }
                        PrimitiveShape::Line { x2, y2 } => {
                            let world_x1 = transform.x;
                            let world_y1 = transform.y;
                            let world_x2 = transform.x + x2;
                            let world_y2 = transform.y + y2;

                            let line_length_squared = (world_x2 - world_x1) * (world_x2 - world_x1)
                                + (world_y2 - world_y1) * (world_y2 - world_y1);

                            if line_length_squared == 0.0 {
                                let dist = ((x - world_x1) * (x - world_x1)
                                    + (y - world_y1) * (y - world_y1))
                                    .sqrt();
                                if dist <= 5.0 {
                                    debug!("Found point entity {} under mouse", entity_id);
                                    return Ok(Some(entity_id));
                                }
                            } else {
                                let t = ((x - world_x1) * (world_x2 - world_x1)
                                    + (y - world_y1) * (world_y2 - world_y1))
                                    / line_length_squared;

                                if (0.0..=1.0).contains(&t) {
                                    let closest_x = world_x1 + t * (world_x2 - world_x1);
                                    let closest_y = world_y1 + t * (world_y2 - world_y1);

                                    let dist = ((x - closest_x) * (x - closest_x)
                                        + (y - closest_y) * (y - closest_y))
                                        .sqrt();

                                    if dist <= 5.0 {
                                        debug!("Found line entity {} under mouse", entity_id);
                                        return Ok(Some(entity_id));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    pub fn start_dragging(
        &self,
        entity_id: u32,
        mouse_x: f32,
        mouse_y: f32,
    ) -> Result<(), &'static str> {
        let (transform, _) = {
            let state = self
                .state
                .try_borrow()
                .map_err(|_| "Failed to borrow game state")?;

            let transform = state
                .transforms
                .get(&entity_id)
                .ok_or("Entity does not have a transform component")?;

            let radius = if let Some(sprite) = state.sprites.get(&entity_id) {
                if let Some(asset) = self.assets.get_by_name(&sprite.asset_name) {
                    if let PrimitiveShape::Circle { radius } = asset.shape {
                        radius
                    } else {
                        return Err("Entity is not a circle");
                    }
                } else {
                    return Err("Asset not found");
                }
            } else {
                return Err("Entity has no sprite");
            };

            (*transform, radius)
        };

        let offset_x = transform.x - mouse_x;
        let offset_y = transform.y - mouse_y;

        println!(
            "Started dragging entity {} at center ({}, {}) with offset ({}, {})",
            entity_id, transform.x, transform.y, offset_x, offset_y
        );

        *self.dragging_state.borrow_mut() = Some(DraggingState {
            entity_id,
            offset_x,
            offset_y,
        });

        Ok(())
    }

    pub fn update_dragged_entity(&self, mouse_x: f32, mouse_y: f32) -> Result<(), &'static str> {
        if let Some(drag_state) = &*self.dragging_state.borrow() {
            let mut state = self
                .state
                .try_borrow_mut()
                .map_err(|_| "Failed to borrow game state")?;

            if let Some(transform) = state.transforms.get_mut(&drag_state.entity_id) {
                transform.x = mouse_x + drag_state.offset_x;
                transform.y = mouse_y + drag_state.offset_y;

                println!(
                    "Updated entity {} center to ({}, {})",
                    drag_state.entity_id, transform.x, transform.y
                );
            }
        }
        Ok(())
    }

    pub fn end_dragging(&self) -> Result<(), &'static str> {
        *self.dragging_state.borrow_mut() = None;
        Ok(())
    }

    pub fn is_entity_dragged(&self, entity_id: u32) -> bool {
        if let Some(drag_state) = &*self.dragging_state.borrow() {
            drag_state.entity_id == entity_id
        } else {
            false
        }
    }
}
