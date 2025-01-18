use super::Manager;
use crate::assets::asset_manager::{AssetManager, PrimitiveShape};
use crate::ecs::components::component::GameState;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct CollisionManager {
    state: Rc<RefCell<GameState>>,
}

impl Manager for CollisionManager {
    fn new(state: Rc<RefCell<GameState>>) -> Self {
        Self { state }
    }

    fn get_state(&self) -> &Rc<RefCell<GameState>> {
        &self.state
    }
}

impl CollisionManager {
    pub fn check_collision(
        &self,
        entity1: u32,
        entity2: u32,
        assets: &AssetManager,
    ) -> Result<bool, &'static str> {
        let state = self
            .state
            .try_borrow()
            .map_err(|_| "Failed to borrow game state")?;

        let transform1 = state
            .transforms
            .get(&entity1)
            .ok_or("Entity 1 has no transform")?;
        let transform2 = state
            .transforms
            .get(&entity2)
            .ok_or("Entity 2 has no transform")?;

        let sprite1 = state
            .sprites
            .get(&entity1)
            .ok_or("Entity 1 has no sprite")?;
        let sprite2 = state
            .sprites
            .get(&entity2)
            .ok_or("Entity 2 has no sprite")?;

        let asset1 = assets
            .get_by_name(&sprite1.asset_name)
            .ok_or("Asset 1 not found")?;
        let asset2 = assets
            .get_by_name(&sprite2.asset_name)
            .ok_or("Asset 2 not found")?;

        match (&asset1.shape, &asset2.shape) {
            (PrimitiveShape::Circle { radius: r1 }, PrimitiveShape::Circle { radius: r2 }) => {
                // For circles, compare distance between centers to sum of radii
                let dx = transform1.x - transform2.x;
                let dy = transform1.y - transform2.y;
                let distance_squared = dx * dx + dy * dy;
                let radii_sum = r1 + r2;
                Ok(distance_squared <= radii_sum * radii_sum)
            }
            (
                PrimitiveShape::Rectangle {
                    width: w1,
                    height: h1,
                },
                PrimitiveShape::Rectangle {
                    width: w2,
                    height: h2,
                },
            ) => {
                // AABB collision for rectangles
                let left1 = transform1.x;
                let right1 = transform1.x + w1;
                let top1 = transform1.y;
                let bottom1 = transform1.y + h1;

                let left2 = transform2.x;
                let right2 = transform2.x + w2;
                let top2 = transform2.y;
                let bottom2 = transform2.y + h2;

                Ok(left1 < right2 && right1 > left2 && top1 < bottom2 && bottom1 > top2)
            }
            (PrimitiveShape::Circle { radius }, PrimitiveShape::Rectangle { width, height }) => {
                // Circle-rectangle collision
                self.check_circle_rectangle_collision(
                    transform1.x,
                    transform1.y,
                    *radius,
                    transform2.x,
                    transform2.y,
                    *width,
                    *height,
                )
            }
            (PrimitiveShape::Rectangle { width, height }, PrimitiveShape::Circle { radius }) => {
                // Rectangle-circle collision (swap order)
                self.check_circle_rectangle_collision(
                    transform2.x,
                    transform2.y,
                    *radius,
                    transform1.x,
                    transform1.y,
                    *width,
                    *height,
                )
            }
            // Add more shape combinations as needed...
            _ => Ok(false), // Default to no collision for unimplemented shape combinations
        }
    }

    pub fn check_entity_tilemap_collision(
        &self,
        entity_id: u32,
        tilemap_id: u32,
        x: f32,
        y: f32,
        assets: &AssetManager,
    ) -> Result<bool, &'static str> {
        let state = self
            .state
            .try_borrow()
            .map_err(|_| "Failed to borrow game state")?;

        let sprite = state
            .sprites
            .get(&entity_id)
            .ok_or("Entity has no sprite")?;
        let asset = assets
            .get_by_name(&sprite.asset_name)
            .ok_or("Asset not found")?;
        let tilemap = state.tilemaps.get(&tilemap_id).ok_or("Tilemap not found")?;

        // Get entity bounds based on shape
        let (entity_left, entity_right, entity_top, entity_bottom) = match &asset.shape {
            PrimitiveShape::Rectangle { width, height } => (x, x + width, y, y + height),
            PrimitiveShape::Circle { radius } => (x - radius, x + radius, y - radius, y + radius),
            // Add handling for other shapes as needed...
            _ => return Ok(false),
        };

        // Convert entity bounds to tile coordinates
        let tile_size = tilemap.tile_size as f32;
        let start_tile_x = (entity_left / tile_size).floor() as u32;
        let end_tile_x = (entity_right / tile_size).ceil() as u32;
        let start_tile_y = (entity_top / tile_size).floor() as u32;
        let end_tile_y = (entity_bottom / tile_size).ceil() as u32;

        // Check each potentially colliding tile
        for tile_y in start_tile_y..=end_tile_y {
            for tile_x in start_tile_x..=end_tile_x {
                if !tilemap.is_walkable(tile_x, tile_y) {
                    // For more precise collision, you could add shape-specific checks here
                    return Ok(true); // Collision detected
                }
            }
        }

        Ok(false) // No collision
    }

    // Helper method for circle-rectangle collision
    fn check_circle_rectangle_collision(
        &self,
        circle_x: f32,
        circle_y: f32,
        radius: f32,
        rect_x: f32,
        rect_y: f32,
        rect_width: f32,
        rect_height: f32,
    ) -> Result<bool, &'static str> {
        // Find closest point on rectangle to circle center
        let closest_x = circle_x.max(rect_x).min(rect_x + rect_width);
        let closest_y = circle_y.max(rect_y).min(rect_y + rect_height);

        // Calculate distance squared between circle center and closest point
        let dx = circle_x - closest_x;
        let dy = circle_y - closest_y;
        let distance_squared = dx * dx + dy * dy;

        // Compare with radius squared
        Ok(distance_squared <= radius * radius)
    }
}
