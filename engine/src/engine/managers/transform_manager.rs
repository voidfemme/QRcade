use std::cell::RefCell;
use std::rc::Rc;

use crate::engine::managers::Manager;
use crate::GameState;

pub struct TransformManager {
    state: Rc<RefCell<GameState>>,
}

impl Manager for TransformManager {
    fn new(state: Rc<RefCell<GameState>>) -> Self {
        Self { state }
    }

    fn get_state(&self) -> &Rc<RefCell<GameState>> {
        &self.state
    }
}

impl TransformManager {
    pub fn set_transform(
        &self,
        entity_id: u32,
        x: f32,
        y: f32,
        rotation: f32,
    ) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                state.add_transform(
                    entity_id,
                    crate::ecs::components::transform::Transform::new(x, y, rotation),
                );
                Ok(())
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn get_transform(&self, entity_id: u32) -> Result<(f32, f32, f32), &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                if let Some(transform) = state.transforms.get(&entity_id) {
                    Ok((transform.x, transform.y, transform.rotation))
                } else {
                    Err("Entity does not have a transform component")
                }
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

}
