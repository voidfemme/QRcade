use crate::ecs::components::component::GameState;
use crate::ecs::systems::collision_system::CollisionSystem;
use crate::ecs::systems::input_system::InputSystem;
use sdl2::keyboard::Keycode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct StateManager {
    pub state: Rc<RefCell<GameState>>,
    input: RefCell<InputSystem>,
}

impl StateManager {
    pub fn new(state: Rc<RefCell<GameState>>) -> Self {
        Self {
            state,
            input: RefCell::new(InputSystem::new()),
        }
    }

    pub fn create_entity(&self) -> Result<u32, &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => Ok(state.create_entity()),
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn destroy_entity(&self, entity_id: u32) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                state.destroy_entity(entity_id);
                Ok(())
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn set_transform(
        &self,
        entity_id: u32,
        x: f32,
        y: f32,
        rotation: f32,
        scale_x: f32,
        scale_y: f32,
    ) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                state.add_transform(
                    entity_id,
                    crate::ecs::components::transform::Transform::new(
                        x, y, rotation, scale_x, scale_y,
                    ),
                );
                Ok(())
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn add_sprite(
        &self,
        entity_id: u32,
        width: f32,
        height: f32,
        r: u8,
        g: u8,
        b: u8,
    ) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                let sprite = crate::ecs::components::sprite::Sprite::new_rectangle(width, height, r, g, b);
                state.add_sprite(entity_id, sprite);
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

    pub fn handle_key(&self, keycode: Keycode, pressed: bool) -> Result<(), &'static str> {
        match self.input.try_borrow_mut() {
            Ok(mut input) => {
                if pressed {
                    input.set_key_pressed(keycode);
                } else {
                    input.set_key_released(keycode);
                }
                Ok(())
            }
            Err(_) => Err("Failed to borrow input system"),
        }
    }

    pub fn is_key_pressed(&self, keycode: Keycode) -> Result<bool, &'static str> {
        match self.input.try_borrow() {
            Ok(input) => Ok(input.is_key_pressed(keycode)),
            Err(_) => Err("Failed to borrow input system"),
        }
    }

    pub fn check_collision(&self, entity1: u32, entity2: u32) -> Result<bool, &'static str> {
        match self.state.try_borrow() {
            Ok(state) => CollisionSystem::are_entities_colliding(&state, entity1, entity2),
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn debug_print_entities(&self) -> Result<(), &'static str> {
        if let Ok(state) = self.state.try_borrow() {
            println!("\nEntity Debug Info:");
            for entity in &state.entities {
                println!("Entity {}: ", entity);
                if let Some(transform) = state.transforms.get(entity) {
                    println!("  Transform: ({}, {})", transform.x, transform.y);
                }
                if let Some(sprite) = state.sprites.get(entity) {
                    println!(
                        "  Sprite: {}x{} RGB({}, {}, {})",
                        sprite.width, sprite.height, sprite.color.0, sprite.color.1, sprite.color.2
                    );
                }
            }
            Ok(())
        } else {
            Err("Failed to borrow game state")
        }
    }
}
