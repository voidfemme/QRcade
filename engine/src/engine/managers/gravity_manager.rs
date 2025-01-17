use crate::ecs::components::gravity::Gravity;
use crate::engine::managers::Manager;
use crate::GameState;

use std::cell::RefCell;
use std::rc::Rc;

pub struct GravityManager {
    state: Rc<RefCell<GameState>>,
}


impl Manager for GravityManager {
    fn new(state: Rc<RefCell<GameState>>) -> Self {
        Self {state}
    }

    fn get_state(&self) -> &Rc<RefCell<GameState>> {
        &self.state
    }
}

impl GravityManager {
    pub fn add_downward_gravity(
        &self,
        entity_id: u32,
        force: f32,
        terminal_velocity: f32,
    ) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if !state.entities.contains(&entity_id) {
                    return Err("Entity does not exist");
                }

                let gravity = Gravity::downward(force, terminal_velocity);
                state.gravities.insert(entity_id, gravity);
                Ok(())
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn add_attractive_gravity(
        &self,
        entity_id: u32,
        force: f32,
        terminal_velocity: f32,
    ) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if !state.entities.contains(&entity_id) {
                    return Err("Entity does not exist");
                }

                let gravity = Gravity::attractive(force, terminal_velocity);
                state.gravities.insert(entity_id, gravity);
                Ok(())
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn add_repulsive_gravity(
        &self,
        entity_id: u32,
        force: f32,
        terminal_velocity: f32,
    ) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if !state.entities.contains(&entity_id) {
                    return Err("Entity does not exist");
                }

                let gravity = Gravity::repulsive(force, terminal_velocity);
                state.gravities.insert(entity_id, gravity);
                Ok(())
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn set_gravity_enabled(&self, entity_id: u32, enabled: bool) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                if let Some(gravity) = state.gravities.get_mut(&entity_id) {
                    gravity.enabled = enabled;
                    Ok(())
                } else {
                    Err("Entity does not have gravity component")
                }
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }


}
