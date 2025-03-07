// ecs/mod.rs
pub mod components;
pub mod systems;

// Re-export frequently used items from components and systems
pub use components::gamestate::GameState;
pub use systems::{
    drag_drop_system::DragDropSystem,
    // other exports as needed
    input_system::InputSystem,
    movement_system::MovementSystem,
    physics::PhysicsSystem,
    rendering::render_system,
};
