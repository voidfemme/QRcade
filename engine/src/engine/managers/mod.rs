use crate::ecs::components::component::GameState;
use std::cell::RefCell;
use std::rc::Rc;

pub mod state_manager;
pub mod collision_manager;
pub mod drag_drop_manager;
pub mod entity_manager;
pub mod gravity_manager;
pub mod input_manager;
pub mod tilemap_manager;
pub mod transform_manager;
pub mod velocity_manager;
pub mod text_manager;

pub trait Manager {
    fn new(state: Rc<RefCell<GameState>>) -> Self
    where
        Self: Sized;
    fn get_state(&self) -> &Rc<RefCell<GameState>>;
}
