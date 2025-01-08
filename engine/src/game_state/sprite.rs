// If I want to do batch rendering or manage a zillion sprites
// at once, that's typically done in a rendering system that
// loops over all entities with a Sprite component and a 
// Transform component, draws them in the correct order, etc.

pub struct Sprite {
    pub texture_id: String,
    // plus other render info
}

impl Sprite {
    pub fn new(texture_id: &str) -> Self {
        Self {
            texture_id: texture_id.to_string(),
            // other fields if needed
        }
    }

    pub fn set_texture(&mut self, texture_id: &str) {
        self.texture_id = texture_id.to_string();
    }
}
