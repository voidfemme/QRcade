use crate::game_state::component::GameState;

pub fn movement_system(game_state: &mut GameState) {
    for (&entity, velocity) in &game_state.velocities {
        if let Some(transform) = game_state.transforms.get_mut(&entity) {
            transform.translate(velocity.dx, velocity.dy);
        }
    }
}
