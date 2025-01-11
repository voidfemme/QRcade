use crate::assets::asset_manager::AssetManager;
use crate::assets::asset_manager::{BuiltInAsset, PrimitiveShape};
use crate::ecs::components::component::GameState;
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

    // Helper function for point-triangle collision
    fn is_point_in_triangle(
        px: f32,
        py: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    ) -> bool {
        // Using barycentric coordinates to determine if point is inside triangle
        let denominator = (y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3);
        let a = ((y2 - y3) * (px - x3) + (x3 - x2) * (py - y3)) / denominator;
        let b = ((y3 - y1) * (px - x3) + (x1 - x3) * (py - y3)) / denominator;
        let c = 1.0 - a - b;

        (0.0..=1.0).contains(&a) && (0.0..=1.0).contains(&b) && (0.0..=1.0).contains(&c)
    }

    fn check_line_intersection(
        start1_x: f32,
        start1_y: f32,
        end1_x: f32,
        end1_y: f32,
        start2_x: f32,
        start2_y: f32,
        end2_x: f32,
        end2_y: f32,
    ) -> bool {
        // Calculate line vectors
        let dx1 = end1_x - start1_x;
        let dy1 = end1_y - start1_y;
        let dx2 = end2_x - start2_x;
        let dy2 = end2_y - start2_y;

        // Calculate cross products
        let cross_prod1 = (end2_x - start1_x) * dy2 - (end2_y - start1_y) * dx2;
        let cross_prod2 = dx1 * dy2 - dy1 * dx2;

        // Check if lines are parallel
        if cross_prod2.abs() < 0.001 {
            return false;
        }

        // Calculate intersection parameters
        let t1 = cross_prod1 / cross_prod2;

        // Check if intersection point lies within both line segments
        (0.0..=1.0).contains(&t1)
    }

    pub fn check_collision(
        transform1: &Transform,
        asset1: &BuiltInAsset,
        transform2: &Transform,
        asset2: &BuiltInAsset,
    ) -> bool {
        println!("Checking collision between:");
        println!("  Entity 1: pos({}, {})", transform1.x, transform1.y);
        println!("  Entity 2: pos({}, {})", transform2.x, transform2.y);

        let result = match (&asset1.shape, &asset2.shape) {
            // Previous collision checks remain...

            // Triangle-Triangle collision
            (
                PrimitiveShape::Triangle {
                    x1,
                    y1,
                    x2,
                    y2,
                    x3,
                    y3,
                },
                PrimitiveShape::Triangle {
                    x1: x4,
                    y1: y4,
                    x2: x5,
                    y2: y5,
                    x3: x6,
                    y3: y6,
                },
            ) => Self::check_triangle_collision(
                transform1, *x1, *y1, *x2, *y2, *x3, *y3, transform2, *x4, *y4, *x5, *y5, *x6, *y6,
            ),

            // Line-Line collision
            (
                PrimitiveShape::Line {
                    x2: end1_x,
                    y2: end1_y,
                },
                PrimitiveShape::Line {
                    x2: end2_x,
                    y2: end2_y,
                },
            ) => Self::check_line_intersection(
                transform1.x,
                transform1.y,
                transform1.x + *end1_x,
                transform1.y + *end1_y,
                transform2.x,
                transform2.y,
                transform2.x + *end2_x,
                transform2.y + *end2_y,
            ),

            // For other combinations, we'll use bounding box collision as a simple approximation
            _ => {
                // Convert all shapes to their bounding rectangles
                let (w1, h1) = match &asset1.shape {
                    PrimitiveShape::Rectangle { width, height } => (*width, *height),
                    PrimitiveShape::Circle { radius } => (*radius * 2.0, *radius * 2.0),
                    PrimitiveShape::Triangle {
                        x1,
                        y1,
                        x2,
                        y2,
                        x3,
                        y3,
                    } => {
                        let min_x = x1.min(x2.min(*x3)).abs();
                        let max_x = x1.max(x2.max(*x3)).abs();
                        let min_y = y1.min(y2.min(*y3)).abs();
                        let max_y = y1.max(y2.max(*y3)).abs();
                        (max_x - min_x, max_y - min_y)
                    }
                    PrimitiveShape::Line { x2, y2 } => (x2.abs(), y2.abs()),
                };

                let (w2, h2) = match &asset2.shape {
                    PrimitiveShape::Rectangle { width, height } => (*width, *height),
                    PrimitiveShape::Circle { radius } => (*radius * 2.0, *radius * 2.0),
                    PrimitiveShape::Triangle {
                        x1,
                        y1,
                        x2,
                        y2,
                        x3,
                        y3,
                    } => {
                        let min_x = x1.min(x2.min(*x3)).abs();
                        let max_x = x1.max(x2.max(*x3)).abs();
                        let min_y = y1.min(y2.min(*y3)).abs();
                        let max_y = y1.max(y2.max(*y3)).abs();
                        (max_x - min_x, max_y - min_y)
                    }
                    PrimitiveShape::Line { x2, y2 } => (x2.abs(), y2.abs()),
                };

                Self::check_rectangle_collision(transform1, w1, h1, transform2, w2, h2)
            }
        };

        if result {
            println!("Collision detected!")
        }

        result
    }
    // Helper function for triangle-triangle collision
    fn check_triangle_collision(
        transform1: &Transform,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        transform2: &Transform,
        x4: f32,
        y4: f32,
        x5: f32,
        y5: f32,
        x6: f32,
        y6: f32,
    ) -> bool {
        // Transform vertices to world space
        let world1_x1 = transform1.x + x1;
        let world1_y1 = transform1.y + y1;
        let world1_x2 = transform1.x + x2;
        let world1_y2 = transform1.y + y2;
        let world1_x3 = transform1.x + x3;
        let world1_y3 = transform1.y + y3;

        let world2_x1 = transform2.x + x4;
        let world2_y1 = transform2.y + y4;
        let world2_x2 = transform2.x + x5;
        let world2_y2 = transform2.y + y5;
        let world2_x3 = transform2.x + x6;
        let world2_y3 = transform2.y + y6;

        // Check if any vertex from one triangle is inside the other
        let any_point_inside = Self::is_point_in_triangle(
            world1_x1, world1_y1, world2_x1, world2_y1, world2_x2, world2_y2, world2_x3, world2_y3,
        ) || Self::is_point_in_triangle(
            world1_x2, world1_y2, world2_x1, world2_y1, world2_x2, world2_y2, world2_x3, world2_y3,
        ) || Self::is_point_in_triangle(
            world1_x3, world1_y3, world2_x1, world2_y1, world2_x2, world2_y2, world2_x3, world2_y3,
        ) || Self::is_point_in_triangle(
            world2_x1, world2_y1, world1_x1, world1_y1, world1_x2, world1_y2, world1_x3, world1_y3,
        ) || Self::is_point_in_triangle(
            world2_x2, world2_y2, world1_x1, world1_y1, world1_x2, world1_y2, world1_x3, world1_y3,
        ) || Self::is_point_in_triangle(
            world2_x3, world2_y3, world1_x1, world1_y1, world1_x2, world1_y2, world1_x3, world1_y3,
        );

        // Check all possible line intersections between triangle edges
        let any_line_intersect = Self::check_line_intersection(
            world1_x1, world1_y1, world1_x2, world1_y2, world2_x1, world2_y1, world2_x2, world2_y2,
        ) || Self::check_line_intersection(
            world1_x2, world1_y2, world1_x3, world1_y3, world2_x1, world2_y1, world2_x2, world2_y2,
        ) || Self::check_line_intersection(
            world1_x3, world1_y3, world1_x1, world1_y1, world2_x1, world2_y1, world2_x2, world2_y2,
        );

        any_point_inside || any_line_intersect
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
            transform1, asset1, transform2, asset2,
        ))
    }
}
