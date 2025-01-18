use std::cell::RefCell;
use std::rc::Rc;

use crate::ecs::components::velocity::Velocity;
use crate::engine::managers::Manager;
use crate::GameState;

#[derive(Debug)]
pub struct VelocityManager {
    state: Rc<RefCell<GameState>>,
}

impl Manager for VelocityManager {
    fn new(state: Rc<RefCell<GameState>>) -> Self {
        Self { state }
    }

    fn get_state(&self) -> &Rc<RefCell<GameState>> {
        &self.state
    }
}

impl VelocityManager {
    pub fn set_velocity(&self, entity_id: u32, vx: f32, vy: f32) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                state.add_velocity(entity_id, Velocity::new(vx, vy));
                Ok(())
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn get_velocity(&self, entity_id: u32) -> Result<(f32, f32), &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                if let Some(velocity) = state.velocities.get(&entity_id) {
                    Ok((velocity.dx, velocity.dy))
                } else {
                    Ok((0.0, 0.0))
                }
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn set_zero_velocity(&self, entity_id: u32) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if let Some(velocity) = state.velocities.get_mut(&entity_id) {
                    *velocity = Velocity::zero();
                    Ok(())
                } else {
                    Err("Entity has no velocity component")
                }
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn set_horizontal_velocity(&self, entity_id: u32, speed: f32) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if let Some(velocity) = state.velocities.get_mut(&entity_id) {
                    *velocity = Velocity::horizontal(speed);
                    Ok(())
                } else {
                    Err("Entity has no velocity component")
                }
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn set_rotation_velocity(
        &self,
        entity_id: u32,
        dx: f32,
        dy: f32,
        angular: f32,
    ) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if let Some(velocity) = state.velocities.get_mut(&entity_id) {
                    *velocity = Velocity::with_rotation(dx, dy, angular);
                    Ok(())
                } else {
                    Err("Entity has no velocity component")
                }
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn set_angular_velocity(&self, entity_id: u32, angular: f32) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if let Some(velocity) = state.velocities.get_mut(&entity_id) {
                    velocity.set_angular(angular);
                    Ok(())
                } else {
                    Err("Entity has no velocity component")
                }
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }
}
