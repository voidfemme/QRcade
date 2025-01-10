use crate::game_state::component::GameState;
use std::cell::RefCell;
use std::rc::Rc;

pub struct StateManager {
    pub state: Rc<RefCell<GameState>>,
}

impl StateManager {
    pub fn new(state: Rc<RefCell<GameState>>) -> Self {
        Self { state }
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
                    crate::game_state::transform::Transform::new(x, y, rotation, scale_x, scale_y),
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
                let sprite =
                    crate::game_state::sprite::Sprite::new_rectangle(width, height, r, g, b);
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
