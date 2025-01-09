pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: f32,
    pub scale: f32,
}

impl Transform {
    pub fn new(x: f32, y: f32, z: f32, rotation: f32, scale: f32) -> Self {
        Self {
            x,
            y,
            z,
            rotation,
            scale,
        }
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}
