// If I want to do batch rendering or manage a zillion sprites
// at once, that's typically done in a rendering system that
// loops over all entities with a Sprite component and a
// Transform component, draws them in the correct order, etc.

#[derive(Clone)]
pub struct Sprite {
    pub width: f32,
    pub height: f32,
    pub color: (u8, u8, u8), // RGB color
}

impl Sprite {
    pub fn new(width: f32, height: f32, r: u8, g: u8, b: u8) -> Self {
        Self {
            width,
            height,
            color: (r, g, b),
            // other fields if needed
        }
    }

    // pub fn set_texture(&mut self, texture_id: &str) {
    //     self.texture_id = texture_id.to_string();
    // }

    pub fn new_rectangle(width: f32, height: f32, r: u8, g: u8, b: u8) -> Self {
        Self {
            width,
            height,
            color: (r, g, b),
        }
    }
}
