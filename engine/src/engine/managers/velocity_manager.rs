use std::cell::RefCell;
use std::rc::Rc;
use crate::ecs::components::velocity::Velocity;
use crate::engine::managers::Manager;
use crate::GameState;
use tracing::{debug, error, warn};

#[derive(Debug)]
pub struct VelocityManager {
   state: Rc<RefCell<GameState>>,
}

impl Manager for VelocityManager {
   fn new(state: Rc<RefCell<GameState>>) -> Self {
       debug!("Creating new VelocityManager");
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
               debug!(
                   entity_id,
                   vx,
                   vy,
                   "Setting entity velocity"
               );
               state.add_velocity(entity_id, Velocity::new(vx, vy));
               Ok(())
           }
           Err(e) => {
               error!(?e, entity_id, "Failed to borrow game state while setting velocity");
               Err("Failed to borrow game state")
           }
       }
   }

   pub fn get_velocity(&self, entity_id: u32) -> Result<(f32, f32), &'static str> {
       match self.state.try_borrow() {
           Ok(state) => {
               match state.velocities.get(&entity_id) {
                   Some(velocity) => {
                       debug!(
                           entity_id,
                           dx = velocity.dx,
                           dy = velocity.dy,
                           "Retrieved entity velocity"
                       );
                       Ok((velocity.dx, velocity.dy))
                   }
                   None => {
                       debug!(entity_id, "No velocity found, returning zero vector");
                       Ok((0.0, 0.0))
                   }
               }
           }
           Err(e) => {
               error!(?e, entity_id, "Failed to borrow game state while getting velocity");
               Err("Failed to borrow game state")
           }
       }
   }

   pub fn set_zero_velocity(&self, entity_id: u32) -> Result<(), &'static str> {
       match self.state.try_borrow_mut() {
           Ok(mut state) => {
               if let Some(velocity) = state.velocities.get_mut(&entity_id) {
                   debug!(entity_id, "Setting zero velocity");
                   *velocity = Velocity::zero();
                   Ok(())
               } else {
                   warn!(entity_id, "Attempted to zero velocity for entity without one");
                   Err("Entity has no velocity component")
               }
           }
           Err(e) => {
               error!(?e, entity_id, "Failed to borrow game state while zeroing velocity");
               Err("Failed to borrow game state")
           }
       }
   }

   pub fn set_horizontal_velocity(&self, entity_id: u32, speed: f32) -> Result<(), &'static str> {
       match self.state.try_borrow_mut() {
           Ok(mut state) => {
               if let Some(velocity) = state.velocities.get_mut(&entity_id) {
                   debug!(entity_id, speed, "Setting horizontal velocity");
                   *velocity = Velocity::horizontal(speed);
                   Ok(())
               } else {
                   warn!(entity_id, "Attempted to set horizontal velocity for entity without one");
                   Err("Entity has no velocity component")
               }
           }
           Err(e) => {
               error!(?e, entity_id, "Failed to borrow game state while setting horizontal velocity");
               Err("Failed to borrow game state")
           }
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
                   debug!(
                       entity_id,
                       dx,
                       dy,
                       angular,
                       "Setting rotation velocity"
                   );
                   *velocity = Velocity::with_rotation(dx, dy, angular);
                   Ok(())
               } else {
                   warn!(entity_id, "Attempted to set rotation velocity for entity without one");
                   Err("Entity has no velocity component")
               }
           }
           Err(e) => {
               error!(?e, entity_id, "Failed to borrow game state while setting rotation velocity");
               Err("Failed to borrow game state")
           }
       }
   }

   pub fn set_angular_velocity(&self, entity_id: u32, angular: f32) -> Result<(), &'static str> {
       match self.state.try_borrow_mut() {
           Ok(mut state) => {
               if let Some(velocity) = state.velocities.get_mut(&entity_id) {
                   debug!(
                       entity_id,
                       angular,
                       "Setting angular velocity"
                   );
                   velocity.set_angular(angular);
                   Ok(())
               } else {
                   warn!(entity_id, "Attempted to set angular velocity for entity without one");
                   Err("Entity has no velocity component")
               }
           }
           Err(e) => {
               error!(?e, entity_id, "Failed to borrow game state while setting angular velocity");
               Err("Failed to borrow game state")
           }
       }
   }
}
