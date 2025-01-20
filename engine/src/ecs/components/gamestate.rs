use super::{
    draggable::Draggable, gravity::Gravity, sprite::Sprite, text::Text, tilemap::Tilemap,
    transform::Transform, velocity::Velocity,
};
use std::collections::HashMap;

pub type Entity = u32;

#[derive(Clone, Debug)]
pub struct GameState {
    pub entities: Vec<Entity>,
    pub transforms: HashMap<Entity, Transform>,
    pub velocities: HashMap<Entity, Velocity>,
    pub sprites: HashMap<Entity, Sprite>,
    pub tilemaps: HashMap<u32, Tilemap>,
    pub gravities: HashMap<Entity, Gravity>,
    pub draggables: HashMap<Entity, Draggable>,
    pub texts: HashMap<Entity, Text>,
}

// ------------------------
// Implement helper methods
// ------------------------
impl GameState {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            transforms: HashMap::new(),
            velocities: HashMap::new(),
            sprites: HashMap::new(),
            tilemaps: HashMap::new(),
            gravities: HashMap::new(),
            draggables: HashMap::new(),
            texts: HashMap::new(),
        }
    }

    // add a draggable component
    pub fn add_draggable(&mut self, entity: Entity, draggable: Draggable) {
        self.draggables.insert(entity, draggable);
    }

    // remove a draggable component
    pub fn remove_draggable(&mut self, entity: Entity) {
        self.draggables.remove(&entity);
    }

    /// Create a new entity and return its include its ID
    pub fn create_entity(&mut self) -> Entity {
        // Simple approach: entity ID is just (length + 1)
        let new_id = self.entities.len() as u32 + 1;
        self.entities.push(new_id);
        new_id
    }

    /// Attach a Velocity component to an entity
    pub fn add_velocity(&mut self, entity: Entity, velocity: Velocity) {
        self.velocities.insert(entity, velocity);
    }

    /// Attach a Transform component to an entity
    pub fn add_transform(&mut self, entity: Entity, transform: Transform) {
        self.transforms.insert(entity, transform);
    }

    /// Attach a Sprite component
    pub fn add_sprite(&mut self, entity: Entity, sprite: Sprite) {
        self.sprites.insert(entity, sprite);
    }

    pub fn _get_velocity(&self, entity_id: u32) -> Option<&Velocity> {
        self.velocities.get(&entity_id)
    }

    /// Destroy an entity
    pub fn destroy_entity(&mut self, entity: Entity) {
        // Remove from entities vector
        if let Some(pos) = self.entities.iter().position(|&e| e == entity) {
            self.entities.remove(pos);
        }

        // Remove all components associated with this entity
        self.transforms.remove(&entity);
        self.velocities.remove(&entity);
        self.sprites.remove(&entity);
        self.draggables.remove(&entity);
    }

    /// Add Text
    pub fn add_text(&mut self, entity: Entity, text: Text) {
        self.texts.insert(entity, text);
    }

    /// Get a reference to a text component
    pub fn get_text(&self, entity: Entity) -> Option<&Text> {
        self.texts.get(&entity)
    }

    /// Get a mutable reference to a Text component
    pub fn get_text_mut(&mut self, entity: Entity) -> Option<&mut Text> {
        self.texts.get_mut(&entity)
    }

    /// Remove a text component from an entity
    pub fn remove_text(&mut self, entity: Entity) {
        self.texts.remove(&entity);
    }

    /// Set the value of the text
    pub fn set_text_value(&mut self, entity_id: u32, value: String) {
        if let Some(text) = self.texts.get_mut(&entity_id) {
            text.value = Some(value);
        }
    }
}
