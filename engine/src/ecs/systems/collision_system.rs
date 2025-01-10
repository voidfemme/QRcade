use crate::ecs::component::GameState;
use crate::ecs::sprite::Sprite;
use crate::ecs::transform::Transform;

pub struct CollisionSystem;

impl CollisionSystem {
    pub fn new() -> Self {
        Self
    }

    pub fn check_collision(
        transform1: &Transform,
        sprite1: &Sprite,
        transform2: &Transform,
        sprite2: &Sprite,
    ) -> bool {
        // First, we calculate the edges of each rectangle.
        // Since our position is at the center, we need to subtract half the width/height
        // to get the left/top edges, and add to get the right/bottom edges
        let left1 = transform1.x - sprite1.width / 2.0;
        let right1 = transform1.x + sprite1.width / 2.0;
        let top1 = transform1.y - sprite1.height / 2.0;
        let bottom1 = transform1.y + sprite1.height / 2.0;

        // Do the same for the second rectangle
        let left2 = transform2.x - sprite2.width / 2.0;
        let right2 = transform2.x + sprite2.width / 2.0;
        let top2 = transform2.y - sprite2.height / 2.0;
        let bottom2 = transform2.y + sprite2.height / 2.0;

        // Now for the actual collision check.
        // Two rectangles overlap if:
        // 1. The left edge of one is less than the right edge of the other (x-axis overlap)
        // 2. The top edge of one is less than the bottom edge of the other (y-axis overlap)
        left1 < right2 && right1 > left2 && top1 < bottom2 && bottom1 > top2
    }

    pub fn are_entities_colliding(
        state: &GameState,
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

        Ok(Self::check_collision(
            transform1, sprite1, transform2, sprite2,
        ))
    }
}
