// If I want to do batch rendering or manage a zillion sprites
// at once, that's typically done in a rendering system that
// loops over all entities with a Sprite component and a
// Transform component, draws them in the correct order, etc.

#[derive(Clone)]
pub struct Sprite {
    pub asset_name: String,  // Reference to the built-in asset this sprite uses
    pub color: (u8, u8, u8), // The sprite's current color, allowing for customization
}

impl Sprite {
    // Create a new sprite from an asset
    pub fn new(asset_name: &str, r: u8, g: u8, b: u8) -> Self {
        Self {
            asset_name: asset_name.to_string(),
            color: (r, g, b),
        }
    }

    // For backward compatibility, create a rectangle sprite
    pub fn new_rectangle(width: f32, height: f32, r: u8, g: u8, b: u8) -> Self {
        // We'll need to make sure "rectangle" is a valid asset name in our asset system
        Self {
            asset_name: "square".to_string(), // Using our built-in square asset
            color: (r, g, b),
        }
    }
}
