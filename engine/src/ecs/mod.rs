// ecs/mod.rs
pub mod components;
pub mod systems;

// Re-export frequently used items from components and systems
pub use components::component::GameState;
pub use systems::{
    movement_system::MovementSystem,
    physics::PhysicsSystem,
    rendering::render_system,
    input_system::InputSystem,
    drag_drop_system::DragDropSystem,
    // other exports as needed
};
