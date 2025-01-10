use crate::lua_api::state_manager::StateManager;
use std::rc::Rc;

pub fn movement_system(state_manager: Rc<StateManager>) {
    // First collect all movements we need to make
    let moves = {
        if let Ok(state) = state_manager.state.try_borrow() {
            state
                .velocities
                .iter()
                .filter_map(|(&entity, velocity)| {
                    state
                        .transforms
                        .get(&entity)
                        .map(|_| (entity, velocity.dx, velocity.dy))
                })
                .collect::<Vec<_>>()
        } else {
            return; // Early return if we can't borrow the state
        }
    };

    // Then apply all the movements
    if let Ok(mut state) = state_manager.state.try_borrow_mut() {
        for (entity, dx, dy) in moves {
            if let Some(transform) = state.transforms.get_mut(&entity) {
                transform.translate(dx, dy);
            }
        }
    }
}

