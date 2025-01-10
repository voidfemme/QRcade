#[derive(Clone, Copy)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}

impl Transform {
    pub fn new(x: f32, y: f32, rotation: f32, scale_x: f32, scale_y: f32) -> Self {
        Self {
            x,
            y,
            rotation,
            scale_x,
            scale_y,
        }
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}
