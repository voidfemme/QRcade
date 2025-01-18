#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

impl Transform {
    pub fn new(x: f32, y: f32, rotation: f32) -> Self {
        Self { x, y, rotation }
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn rotate(&mut self, angle: f32) {
        // Add the new angle to the current rotation
        self.rotation += angle;

        // Normalize the angle to keep it between 0 and 2PI
        while self.rotation > std::f32::consts::TAU {
            self.rotation -= std::f32::consts::TAU;
        }
        while self.rotation < 0.0 {
            self.rotation += std::f32::consts::TAU;
        }
    }
}
