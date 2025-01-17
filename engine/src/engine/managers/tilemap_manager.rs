use super::Manager;
use crate::ecs::components::component::GameState;
use crate::ecs::components::tilemap::{Tile, Tilemap, TilemapQuery, TilemapQueryResult};
use std::cell::RefCell;
use std::rc::Rc;

pub struct TilemapManager {
    state: Rc<RefCell<GameState>>,
}

impl Manager for TilemapManager {
    fn new(state: Rc<RefCell<GameState>>) -> Self {
        Self { state }
    }

    fn get_state(&self) -> &Rc<RefCell<GameState>> {
        &self.state
    }
}

impl TilemapManager {
    pub fn create_tilemap(
        &self,
        entity_id: u32,
        width: u32,
        height: u32,
        tile_size: u32,
    ) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                // Check if entity exists
                if !state.entities.contains(&entity_id) {
                    return Err("Entity does not exist");
                }

                // Create new tilemap
                let tilemap = Tilemap::new(width, height, tile_size);

                // Add tilemap to state
                state.tilemaps.insert(entity_id, tilemap);
                Ok(())
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn get_tilemap(&self, entity_id: u32) -> Result<Option<Tilemap>, &'static str> {
        match self.state.try_borrow() {
            Ok(state) => Ok(state.tilemaps.get(&entity_id).cloned()),
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn set_tile(
        &self,
        entity_id: u32,
        x: u32,
        y: u32,
        tile_id: u32,
        walkable: bool,
        color: (u8, u8, u8),
    ) -> Result<(), &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                // Get tilemap for entity
                let tilemap = state
                    .tilemaps
                    .get_mut(&entity_id)
                    .ok_or("No tilemap found for entity")?;

                // Create new tile
                let tile = Tile {
                    tile_id,
                    walkable,
                    color,
                };

                // Set tile in tilemap
                tilemap.set_tile(x, y, tile)
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn clear_tile(&self, entity_id: u32, x: u32, y: u32) -> Result<bool, &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                // Get tilemap for entity
                let tilemap = state
                    .tilemaps
                    .get_mut(&entity_id)
                    .ok_or("No tilemap found for entity")?;

                // Get the walkable status before clearing
                let was_walkable = tilemap.is_walkable(x, y);

                // Clear the tile
                tilemap.clear_tile(x, y)?;

                // Return the previous walkable status
                Ok(was_walkable)
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn query_tilemap(
        &self,
        entity_id: u32,
        query: TilemapQuery,
    ) -> Result<TilemapQueryResult, &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                let tilemap = state
                    .tilemaps
                    .get(&entity_id)
                    .ok_or("No tilemap found for entity")?;
                tilemap.query(query)
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn is_tile_walkable(&self, entity_id: u32, x: u32, y: u32) -> Result<bool, &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                // Get tilemap for entity
                let tilemap = state
                    .tilemaps
                    .get(&entity_id)
                    .ok_or("No tilemap found for entity")?;

                // Return walkable status
                Ok(tilemap.is_walkable(x, y))
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn check_position_walkable(
        &self,
        entity_id: u32,
        tilemap_id: u32,
        x: f32,
        y: f32,
        collision_manager: &super::collision_manager::CollisionManager,
        assets: &crate::assets::asset_manager::AssetManager,
    ) -> Result<bool, &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                // First check if the tilemap exists
                if state.tilemaps.contains_key(&tilemap_id) {
                    // Check for collisions using the collision manager
                    match collision_manager
                        .check_entity_tilemap_collision(entity_id, tilemap_id, x, y, assets)
                    {
                        Ok(collision) => Ok(!collision), // If there's no collision, it's walkable
                        Err(e) => Err(e),
                    }
                } else {
                    Err("Tilemap not found")
                }
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn get_tile_size(&self, entity_id: u32) -> Result<u32, &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                let tilemap = state
                    .tilemaps
                    .get(&entity_id)
                    .ok_or("No tilemap found for entity")?;
                Ok(tilemap.tile_size)
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }

    pub fn get_dimensions(&self, entity_id: u32) -> Result<(u32, u32), &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                let tilemap = state
                    .tilemaps
                    .get(&entity_id)
                    .ok_or("No tilemap found for entity")?;
                Ok((tilemap.width, tilemap.height))
            }
            Err(_) => Err("Failed to borrow game state"),
        }
    }
}

