use std::cell::RefCell;
use std::rc::Rc;
use crate::engine::managers::Manager;
use crate::GameState;
use tracing::{debug, error, warn};

#[derive(Debug)]
pub struct TransformManager {
   state: Rc<RefCell<GameState>>,
}

impl Manager for TransformManager {
   fn new(state: Rc<RefCell<GameState>>) -> Self {
       debug!("Creating new TransformManager");
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
               debug!(
                   entity_id,
                   x,
                   y,
                   rotation,
                   "Setting entity transform"
               );
               
               state.add_transform(
                   entity_id,
                   crate::ecs::components::transform::Transform::new(x, y, rotation),
               );
               
               debug!(entity_id, "Transform set successfully");
               Ok(())
           }
           Err(e) => {
               error!(?e, entity_id, "Failed to borrow game state while setting transform");
               Err("Failed to borrow game state")
           }
       }
   }

   pub fn get_transform(&self, entity_id: u32) -> Result<(f32, f32, f32), &'static str> {
       match self.state.try_borrow() {
           Ok(state) => {
               match state.transforms.get(&entity_id) {
                   Some(transform) => {
                       debug!(
                           entity_id,
                           x = transform.x,
                           y = transform.y,
                           rotation = transform.rotation,
                           "Retrieved entity transform"
                       );
                       Ok((transform.x, transform.y, transform.rotation))
                   }
                   None => {
                       warn!(entity_id, "Attempted to get transform for entity without one");
                       Err("Entity does not have a transform component")
                   }
               }
           }
           Err(e) => {
               error!(?e, entity_id, "Failed to borrow game state while getting transform");
               Err("Failed to borrow game state")
           }
       }
   }
}
