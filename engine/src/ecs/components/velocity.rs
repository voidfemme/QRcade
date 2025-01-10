// for Velocity component

#[derive(Clone, Copy)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

impl Velocity {
    pub fn new(dx: f32, dy: f32) -> Self {
        Self { dx, dy }
    }

    pub fn set_velocity(&mut self, dx: f32, dy: f32) {
        self.dx = dx;
        self.dy = dy;
    }
}
