// for Velocity component

#[derive(Clone, Copy, Debug)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
    pub angular: f32,
}

impl Velocity {
    pub fn new(dx: f32, dy: f32) -> Self {
        Self {
            dx,
            dy,
            angular: 0.0,
        }
    }

    pub fn zero() -> Self {
        Self {
            dx: 0.0,
            dy: 0.0,
            angular: 0.0,
        }
    }

    // Helper function for typical horizontal movement speeds
    pub fn horizontal(speed: f32) -> Self {
        Self {
            dx: speed,
            dy: 0.0,
            angular: 0.0,
        }
    }

    pub fn with_rotation(dx: f32, dy: f32, angular: f32) -> Self {
        Self { dx, dy, angular }
    }

    pub fn set_velocity(&mut self, dx: f32, dy: f32) {
        self.dx = dx;
        self.dy = dy;
    }

    pub fn set_angular(&mut self, angular: f32) {
        self.angular = angular;
    }
}
