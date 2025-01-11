// ecs/mod.rs
pub mod systems;
pub mod components;

// Re-export frequently used items from components and systems
pub use components::component::GameState;
pub use systems::{
    movement_system::movement_system,
    rendering::render_system,
    // other exports as needed
};
