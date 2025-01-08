use crate::GameState;

pub fn physics_system(state: &mut GameState) {
    // For every entity that has both a Transform and a Velocity...
    for (&entity, velocity) in &state.velocities {
        if let Some(transform) = state.transforms.get_mut(&entity) {
            // Move the transform
            transform.translate(velocity.dx, velocity.dy);
        }
    }
}
