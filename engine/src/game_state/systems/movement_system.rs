use crate::game_state::component::GameState;
use std::cell::RefCell;
use std::rc::Rc;

pub fn movement_system(gamestate: Rc<RefCell<GameState>>) {
    let mut state = gamestate.borrow_mut(); // Borrow mutable access to the GameState

    // Create a vector of moves to avoid borrow checker issues
    let moves: Vec<_> = state
        .velocities
        .iter()
        .filter_map(|(&entity, velocity)| {
            state
                .transforms
                .get(&entity)
                .map(|_transform| (entity, velocity.dx, velocity.dy))
        })
        .collect();

    // Apply the moves
    for (entity, dx, dy) in moves {
        if let Some(transform) = state.transforms.get_mut(&entity) {
            transform.translate(dx, dy);
        }
    }
}
