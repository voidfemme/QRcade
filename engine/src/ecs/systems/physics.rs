use crate::GameState;

pub fn physics_system(state: &mut GameState, delta_time: f32) {
    // For every entity that has both a Transform and a Velocity...
    for (&entity, velocity) in &state.velocities {
        if let Some(transform) = state.transforms.get_mut(&entity) {
            // Apply linear motion
            transform.translate(velocity.dx * delta_time, velocity.dy * delta_time);

            // Apply angular motion
            transform.rotate(velocity.angular * delta_time);
        }
    }
}

