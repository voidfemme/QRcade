use crate::engine::managers::state_manager::StateManager;
use std::rc::Rc;

/// MovementSystem handles direct movement commands and updates.
/// It focuses on immediate movement controls rather than physics simulation.
pub struct MovementSystem {
    state_manager: Rc<StateManager>,
}

impl MovementSystem {
    pub fn new(state_manager: Rc<StateManager>) -> Self {
        Self { state_manager }
    }

    pub fn update(&self, delta_time: f32) {
        // Process movement commands
        if let Ok(mut state) = self.state_manager.state.try_borrow_mut() {
            for (&entity_id, velocity) in state.velocities.iter_mut() {
                // Apply any direct velocity changes from commands
                if let Some(command) = self.get_pending_movement_command(entity_id) {
                    match command {
                        MovementCommand::SetVelocity { dx, dy } => {
                            velocity.dx = dx;
                            velocity.dy = dy;
                        }
                        MovementCommand::SetHorizontal(speed) => {
                            velocity.dx = speed;
                        }
                        MovementCommand::SetVertical(speed) => {
                            velocity.dy = speed;
                        }
                        MovementCommand::SetAngular(speed) => {
                            velocity.angular = speed;
                        }
                        MovementCommand::Stop => {
                            velocity.dx = 0.0;
                            velocity.dy = 0.0;
                            velocity.angular = 0.0;
                        }
                    }
                }

                // Apply friction if configured
                if let Some(friction) = self.get_friction(entity_id) {
                    velocity.dx *= friction.powf(delta_time);
                    velocity.dy *= friction.powf(delta_time);
                    velocity.angular *= friction.powf(delta_time);
                }
            }
        }
    }

    /// Retrieves any pending movement command for an entity
    fn get_pending_movement_command(&self, _entity_id: u32) -> Option<MovementCommand> {
        // This would interface with your command queue or input system
        // For now, we'll return None as a placeholder
        None
    }

    /// Gets the friction coefficient for an entity if it has one
    fn get_friction(&self, _entity_id: u32) -> Option<f32> {
        // This could be expanded to look up friction from a component
        // For now we'll return a default friction
        Some(0.98)
    }
}

/// Represents different types of movement commands that can be issued
#[derive(Debug, Clone)]
pub enum MovementCommand {
    SetVelocity { dx: f32, dy: f32 },
    SetHorizontal(f32),
    SetVertical(f32),
    SetAngular(f32),
    Stop,
}
