use crate::ecs::components::gravity::GravityType;
use crate::lua::runtime::state_manager::StateManager;
use std::rc::Rc;

pub fn movement_system(state_manager: Rc<StateManager>, delta_time: f32) {
    // First collect all velocity changes from gravity
    let gravity_updates = {
        if let Ok(state) = state_manager.state.try_borrow() {
            state
                .gravities
                .iter()
                .filter_map(|(&entity_id, gravity)| {
                    if !gravity.enabled {
                        return None;
                    }

                    state.velocities.get(&entity_id).map(|velocity| {
                        let (dx, dy) = match gravity.gravity_type {
                            GravityType::Downward => {
                                // Only affect vertical velocity
                                let new_dy = (velocity.dy + gravity.force * delta_time)
                                    .min(gravity.terminal_velocity);
                                (velocity.dx, new_dy)
                            }
                            GravityType::Attractive | GravityType::Repulsive => {
                                // Get entity position
                                if let Some(transform) = state.transforms.get(&entity_id) {
                                    // Calculate gravitational influence on other entities
                                    let mut total_dx = velocity.dx;
                                    let mut total_dy = velocity.dy;

                                    // Look at all other entities
                                    for (&other_id, other_transform) in &state.transforms {
                                        if other_id != entity_id {
                                            let dx = other_transform.x - transform.x;
                                            let dy = other_transform.y - transform.y;
                                            let distance_squared = dx * dx + dy * dy;

                                            if distance_squared > 0.0001 {
                                                // Avoid division by zero
                                                let distance = distance_squared.sqrt();
                                                let force_magnitude =
                                                    gravity.force / distance_squared;

                                                // Normalize direction
                                                let dir_x = dx / distance;
                                                let dir_y = dy / distance;

                                                // Attractive pulls toward, Repulsive pushes away
                                                let multiplier = match gravity.gravity_type {
                                                    GravityType::Attractive => 1.0,
                                                    GravityType::Repulsive => -1.0,
                                                    _ => unreachable!(),
                                                };

                                                total_dx += dir_x
                                                    * force_magnitude
                                                    * multiplier
                                                    * delta_time;
                                                total_dy += dir_y
                                                    * force_magnitude
                                                    * multiplier
                                                    * delta_time;
                                            }
                                        }
                                    }

                                    // Apply terminal velocity to both directions for attraction/repulsion
                                    let speed = (total_dx * total_dx + total_dy * total_dy).sqrt();
                                    if speed > gravity.terminal_velocity {
                                        let scale = gravity.terminal_velocity / speed;
                                        total_dx *= scale;
                                        total_dy *= scale;
                                    }

                                    (total_dx, total_dy)
                                } else {
                                    (velocity.dx, velocity.dy)
                                }
                            }
                        };
                        (entity_id, dx, dy)
                    })
                })
                .collect::<Vec<_>>()
        } else {
            return;
        }
    };

    // Apply gravity updates to velocities
    if let Ok(mut state) = state_manager.state.try_borrow_mut() {
        for (entity, dx, dy) in gravity_updates {
            if let Some(velocity) = state.velocities.get_mut(&entity) {
                velocity.dx = dx;
                velocity.dy = dy;
            }
        }
    }

    // Then collect all movements we need to make based on final velocities
    let moves = {
        if let Ok(state) = state_manager.state.try_borrow() {
            state
                .velocities
                .iter()
                .filter_map(|(&entity, velocity)| {
                    state
                        .transforms
                        .get(&entity)
                        .map(|_| (entity, velocity.dx * delta_time, velocity.dy * delta_time))
                })
                .collect::<Vec<_>>()
        } else {
            return;
        }
    };

    // Finally apply all the movements
    if let Ok(mut state) = state_manager.state.try_borrow_mut() {
        for (entity, dx, dy) in moves {
            if let Some(transform) = state.transforms.get_mut(&entity) {
                transform.translate(dx, dy);
            }
        }
    }
}

