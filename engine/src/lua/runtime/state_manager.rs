use crate::assets::asset_manager::{AssetManager, BuiltInAsset, PrimitiveShape};
use crate::ecs::components::component::GameState;
use crate::ecs::systems::collision_system::CollisionSystem;
use crate::ecs::systems::input_system::InputSystem;
use crate::{Renderer, Sdl2Renderer};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::cell::RefCell;
use std::rc::Rc;

pub struct StateManager {
    pub state: Rc<RefCell<GameState>>,
    input: RefCell<InputSystem>,
    assets: AssetManager,
}

impl StateManager {
    pub fn new(state: Rc<RefCell<GameState>>) -> Self {
        Self {
            state,
            input: RefCell::new(InputSystem::new()),
            assets: AssetManager::new(),
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
        asset_name: &str,
        r: u8,
        g: u8,
        b: u8,
    ) -> Result<(), &'static str> {
        // First verify the asset exists
        if self.get_asset(asset_name).is_none() {
            return Err("Asset not found");
        }

        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                // Create a sprite that references our asset
                let sprite = crate::ecs::components::sprite::Sprite::new(asset_name, r, g, b);
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

    pub fn get_asset(&self, name: &str) -> Option<&BuiltInAsset> {
        println!("Attempting to get asset with name: '{}'", name);
        let result = self.assets.get_by_name(name);
        println!(
            "Result: {}",
            if result.is_some() {
                "found"
            } else {
                "not found"
            }
        );
        result
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
            Ok(state) => {
                CollisionSystem::are_entities_colliding(&state, &self.assets, entity1, entity2)
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn render_asset(
        &self,
        asset: &BuiltInAsset,
        x: i32,
        y: i32,
        color: (u8, u8, u8),
        renderer: &mut Sdl2Renderer,
        scale: f32,
    ) {
        match &asset.shape {
            PrimitiveShape::Rectangle { width, height } => {
                renderer.draw_scaled_rect(
                    x,
                    y,
                    (*width * scale) as u32,
                    (*height * scale) as u32,
                    scale,
                    color,
                    true, // Keep debug visualization on
                );
            }
            PrimitiveShape::Circle { radius } => {
                let scaled_radius = (*radius * scale) as u32;
                renderer.draw_circle(
                    x + scaled_radius as i32,
                    y + scaled_radius as i32,
                    scaled_radius,
                    Color::RGB(color.0, color.1, color.2),
                );

                // If debug visualization is enabled, we might want to draw
                // a bounding box around the Circle
                if true {
                    // Debug visualization
                    let diameter = *radius * 2.0 * scale;
                    renderer.draw_bounding_box(
                        x + (diameter as i32 / 2),
                        y + (diameter as i32 / 2),
                        diameter as u32,
                        diameter as u32,
                        Color::RGB(255, 255, 255),
                    );
                }
            }
        }
    }

    pub fn debug_print_entities(&self) -> Result<(), &'static str> {
        if let Ok(state) = self.state.try_borrow() {
            println!("\nEntity Debug Info:");
            for entity in &state.entities {
                println!("Entity {}: ", entity);

                // Print transform information
                if let Some(transform) = state.transforms.get(entity) {
                    println!("  Transform: ({}, {})", transform.x, transform.y);
                }

                // Print sprite information, now including the asset type
                if let Some(sprite) = state.sprites.get(entity) {
                    // Look up the asset to get shape information
                    if let Some(asset) = self.get_asset(&sprite.asset_name) {
                        // Print different information based on the shape type
                        match &asset.shape {
                            PrimitiveShape::Rectangle { width, height } => {
                                println!(
                                    "  Sprite: Rectangle {}x{} RGB({}, {}, {})",
                                    width, height, sprite.color.0, sprite.color.1, sprite.color.2
                                );
                            }
                            PrimitiveShape::Circle { radius } => {
                                println!(
                                    "  Sprite: Circle radius {} RGB({}, {}, {})",
                                    radius, sprite.color.0, sprite.color.1, sprite.color.2
                                );
                            }
                        }
                    } else {
                        println!(
                            "  Sprite: Unknown asset '{}' RGB({}, {}, {})",
                            sprite.asset_name, sprite.color.0, sprite.color.1, sprite.color.2
                        );
                    }
                }
            }
            Ok(())
        } else {
            Err("Failed to borrow game state")
        }
    }
}
