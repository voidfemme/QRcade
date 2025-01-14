#[derive(Clone, Debug)]
pub enum GravityType {
    Downward,   // Traditional downward gravity
    Attractive, // Like a black hole
    Repulsive,  // Like an explosion
}

#[derive(Clone, Debug)]
pub struct Gravity {
    pub force: f32,             // Downward force
    pub enabled: bool,          // Whether gravity affects this entity
    pub terminal_velocity: f32, // Max fall speed
    pub gravity_type: GravityType,
}

impl Gravity {
    pub fn new(force: f32, terminal_velocity: f32, gravity_type: GravityType) -> Self {
        Self {
            force,
            enabled: true,
            terminal_velocity,
            gravity_type,
        }
    }

    pub fn default() -> Self {
        Self {
            force: 9.8, // Standard gravity
            enabled: true,
            terminal_velocity: 10.0,
            gravity_type: GravityType::Downward,
        }
    }

    pub fn downward(force: f32, terminal_velocity: f32) -> Self {
        Self::new(force, terminal_velocity, GravityType::Downward)
    }

    pub fn attractive(force: f32, terminal_velocity: f32) -> Self {
        Self::new(force, terminal_velocity, GravityType::Attractive)
    }

    pub fn repulsive(force: f32, terminal_velocity: f32) -> Self {
        Self::new(force, terminal_velocity, GravityType::Repulsive)
    }
}
