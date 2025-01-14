use super::{
    gravity::Gravity, sprite::Sprite, tilemap::Tilemap, transform::Transform, velocity::Velocity,
};
use std::collections::HashMap;

pub type Entity = u32;

#[derive(Clone, Copy)]
pub struct Collider {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone)]
pub struct GameState {
    pub entities: Vec<Entity>,
    pub colliders: Vec<Collider>,
    pub transforms: HashMap<Entity, Transform>,
    pub velocities: HashMap<Entity, Velocity>,
    pub sprites: HashMap<Entity, Sprite>,
    pub tilemaps: HashMap<u32, Tilemap>,
    pub gravities: HashMap<Entity, Gravity>,
    next_entity_id: u32,
}

// ------------------------
// Implement helper methods
// ------------------------
impl GameState {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            colliders: Vec::new(),
            transforms: HashMap::new(),
            velocities: HashMap::new(),
            sprites: HashMap::new(),
            tilemaps: HashMap::new(),
            gravities: HashMap::new(),
            next_entity_id: 0,
        }
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

    /// Attach a Collider component
    pub fn add_collider(&mut self, collider: Collider) {
        self.colliders.push(collider);
    }

    pub fn get_velocity(&self, entity_id: u32) -> Option<&Velocity> {
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
    }
}
