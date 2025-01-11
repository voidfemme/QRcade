// If I want to do batch rendering or manage a zillion sprites
// at once, that's typically done in a rendering system that
// loops over all entities with a Sprite component and a
// Transform component, draws them in the correct order, etc.

#[derive(Clone)]
pub enum SpriteShapeData {
    Rectangle { width: f32, height: f32 },
    Circle { radius: f32 },
    Triangle { vertices: [(f32, f32); 3] },
    Line { end: (f32, f32) },
}

#[derive(Clone)]
pub struct Sprite {
    pub asset_name: String,  // Reference to the built-in asset this sprite uses
    pub color: (u8, u8, u8), // The sprite's current color, allowing for customization
    pub shape_data: Option<SpriteShapeData>,
}

impl Sprite {
    // Create a new sprite from an asset
    pub fn new(asset_name: &str, r: u8, g: u8, b: u8, shape_data: Option<SpriteShapeData>) -> Self {
        Self {
            asset_name: asset_name.to_string(),
            color: (r, g, b),
            shape_data,
        }
    }

    // For backward compatibility, create a rectangle sprite
    pub fn new_rectangle(width: f32, height: f32, r: u8, g: u8, b: u8) -> Self {
        let shape_data = Some(SpriteShapeData::Rectangle { width, height });
        // We'll need to make sure "rectangle" is a valid asset name in our asset system
        Self {
            asset_name: "square".to_string(), // Using our built-in square asset
            color: (r, g, b),
            shape_data,
        }
    }
    pub fn new_circle(radius: f32, r: u8, g: u8, b: u8) -> Self {
        let shape_data = Some(SpriteShapeData::Circle { radius });
        Self {
            asset_name: "circle".to_string(),
            color: (r, g, b),
            shape_data,
        }
    }
    pub fn new_triangle(
        r: u8,
        g: u8,
        b: u8,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    ) -> Self {
        let shape_data = Some(SpriteShapeData::Triangle {
            vertices: [(x1, y1), (x2, y2), (x3, y3)],
        });
        Self {
            asset_name: "triangle".to_string(),
            color: (r, g, b),
            shape_data,
        }
    }
    pub fn new_line(r: u8, g: u8, b: u8, x: f32, y: f32) -> Self {
        let shape_data = Some(SpriteShapeData::Line { end: (x, y) });
        Self {
            asset_name: "line".to_string(),
            color: (r, g, b),
            shape_data,
        }
    }
}
