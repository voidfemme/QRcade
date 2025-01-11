use crate::assets::asset_manager::AssetManager;
use crate::assets::asset_manager::{BuiltInAsset, PrimitiveShape};
use crate::ecs::components::component::GameState;
use crate::ecs::components::sprite::Sprite;
use crate::ecs::components::transform::Transform;

pub struct CollisionSystem;

impl CollisionSystem {
    pub fn new() -> Self {
        Self
    }

    // Helper function for rectangle-rectangle collision
    fn check_rectangle_collision(
        transform1: &Transform,
        width1: f32,
        height1: f32,
        transform2: &Transform,
        width2: f32,
        height2: f32,
    ) -> bool {
        let left1 = transform1.x - width1 / 2.0;
        let right1 = transform1.x + width1 / 2.0;
        let top1 = transform1.y - height1 / 2.0;
        let bottom1 = transform1.y + height1 / 2.0;

        let left2 = transform2.x - width2 / 2.0;
        let right2 = transform2.x + width2 / 2.0;
        let top2 = transform2.y - height2 / 2.0;
        let bottom2 = transform2.y + height2 / 2.0;

        left1 < right2 && right1 > left2 && top1 < bottom2 && bottom1 > top2
    }

    // Helper function for circle-circle collision
    fn check_circle_collision(
        transform1: &Transform,
        radius1: f32,
        transform2: &Transform,
        radius2: f32,
    ) -> bool {
        // Calculate the distance between circle centers
        let dx = transform2.x - transform1.x;
        let dy = transform2.y - transform1.y;
        let distance_squared = dx * dx + dy * dy;

        // Compare with sum of radii squared (avoiding square root for performance)
        let radii_sum = radius1 + radius2;
        distance_squared <= radii_sum * radii_sum
    }

    pub fn check_collision(
        transform1: &Transform,
        sprite1: &Sprite,
        asset1: &BuiltInAsset,
        transform2: &Transform,
        sprite2: &Sprite,
        asset2: &BuiltInAsset,
    ) -> bool {
        println!("Checking collision between:");
        println!("  Entity 1: pos({}, {})", transform1.x, transform1.y);
        println!("  Entity 2: pos({}, {})", transform2.x, transform2.y);

        let result = match (&asset1.shape, &asset2.shape) {
            // Rectangle-Rectangle collision
            (
                PrimitiveShape::Rectangle {
                    width: w1,
                    height: h1,
                },
                PrimitiveShape::Rectangle {
                    width: w2,
                    height: h2,
                },
            ) => Self::check_rectangle_collision(transform1, *w1, *h1, transform2, *w2, *h2),

            // Circle-Circle collision
            (PrimitiveShape::Circle { radius: r1 }, PrimitiveShape::Circle { radius: r2 }) => {
                Self::check_circle_collision(transform1, *r1, transform2, *r2)
            }

            // For mixed shape collisions (circle-rectangle), we can either:
            // 1. Use rectangle collision as an approximation (simpler but less accurate)
            // 2. Implement specific circle-rectangle collision (more accurate but more complex)
            // For now, let's use rectangle approximation for mixed shapes
            _ => {
                // Convert circles to rectangles for approximation
                let (w1, h1) = match &asset1.shape {
                    PrimitiveShape::Rectangle { width, height } => (*width, *height),
                    PrimitiveShape::Circle { radius } => (*radius * 2.0, *radius * 2.0),
                };

                let (w2, h2) = match &asset2.shape {
                    PrimitiveShape::Rectangle { width, height } => (*width, *height),
                    PrimitiveShape::Circle { radius } => (*radius * 2.0, *radius * 2.0),
                };

                Self::check_rectangle_collision(transform1, w1, h1, transform2, w2, h2)
            }
        };

        if result {
            println!("Collision detected!")
        }

        result
    }

    pub fn are_entities_colliding(
        state: &GameState,
        asset_manager: &AssetManager,
        entity1: u32,
        entity2: u32,
    ) -> Result<bool, &'static str> {
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

        let asset1 = asset_manager
            .get_by_name(&sprite1.asset_name)
            .ok_or("Could not find asset for Entity 1")?;
        let asset2 = asset_manager
            .get_by_name(&sprite2.asset_name)
            .ok_or("Could not find asset for Entity 2")?;

        Ok(Self::check_collision(
            transform1, sprite1, asset1, transform2, sprite2, asset2,
        ))
    }
}

