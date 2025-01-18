use super::Manager;
use crate::ecs::components::component::GameState;
use crate::ecs::components::tilemap::{Tile, Tilemap, TilemapQuery, TilemapQueryResult};
use std::cell::RefCell;
use std::rc::Rc;
use tracing::{debug, error, warn};

#[derive(Debug)]
pub struct TilemapManager {
    state: Rc<RefCell<GameState>>,
}

impl Manager for TilemapManager {
    fn new(state: Rc<RefCell<GameState>>) -> Self {
        debug!("Creating new TilemapManager");
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
                if !state.entities.contains(&entity_id) {
                    error!(
                        entity_id,
                        "Attempted to create tilemap for non-existent entity"
                    );
                    return Err("Entity does not exist");
                }

                debug!(entity_id, width, height, tile_size, "Creating new tilemap");
                let tilemap = Tilemap::new(width, height, tile_size);
                state.tilemaps.insert(entity_id, tilemap);
                Ok(())
            }
            Err(e) => {
                error!(
                    ?e,
                    entity_id, "Failed to borrow game state while creating tilemap"
                );
                Err("Failed to borrow game state")
            }
        }
    }

    pub fn get_tilemap(&self, entity_id: u32) -> Result<Option<Tilemap>, &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                let has_tilemap = state.tilemaps.contains_key(&entity_id);
                debug!(entity_id, has_tilemap, "Retrieved tilemap");
                Ok(state.tilemaps.get(&entity_id).cloned())
            }
            Err(e) => {
                error!(
                    ?e,
                    entity_id, "Failed to borrow game state while getting tilemap"
                );
                Err("Failed to borrow game state")
            }
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
                let tilemap = match state.tilemaps.get_mut(&entity_id) {
                    Some(tm) => tm,
                    None => {
                        warn!(entity_id, "No tilemap found for entity when setting tile");
                        return Err("No tilemap found for entity");
                    }
                };

                debug!(
                    entity_id,
                    x,
                    y,
                    tile_id,
                    walkable,
                    r = color.0,
                    g = color.1,
                    b = color.2,
                    "Setting tile"
                );

                let tile = Tile {
                    tile_id,
                    walkable,
                    color,
                };

                tilemap.set_tile(x, y, tile)
            }
            Err(e) => {
                error!(
                    ?e,
                    entity_id, "Failed to borrow game state while setting tile"
                );
                Err("Failed to borrow game state")
            }
        }
    }

    pub fn clear_tile(&self, entity_id: u32, x: u32, y: u32) -> Result<bool, &'static str> {
        match self.state.try_borrow_mut() {
            Ok(mut state) => {
                let tilemap = match state.tilemaps.get_mut(&entity_id) {
                    Some(tm) => tm,
                    None => {
                        warn!(entity_id, "No tilemap found for entity when clearing tile");
                        return Err("No tilemap found for entity");
                    }
                };

                let was_walkable = tilemap.is_walkable(x, y);
                debug!(entity_id, x, y, was_walkable, "Clearing tile");
                tilemap.clear_tile(x, y)?;

                Ok(was_walkable)
            }
            Err(e) => {
                error!(
                    ?e,
                    entity_id, "Failed to borrow game state while clearing tile"
                );
                Err("Failed to borrow game state")
            }
        }
    }

    pub fn query_tilemap(
        &self,
        entity_id: u32,
        query: TilemapQuery,
    ) -> Result<TilemapQueryResult, &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                let tilemap = match state.tilemaps.get(&entity_id) {
                    Some(tm) => tm,
                    None => {
                        warn!(
                            entity_id,
                            ?query,
                            "No tilemap found for entity during query"
                        );
                        return Err("No tilemap found for entity");
                    }
                };

                debug!(entity_id, ?query, "Querying tilemap");
                tilemap.query(query)
            }
            Err(e) => {
                error!(
                    ?e,
                    entity_id, "Failed to borrow game state during tilemap query"
                );
                Err("Failed to borrow game state")
            }
        }
    }

    pub fn is_tile_walkable(&self, entity_id: u32, x: u32, y: u32) -> Result<bool, &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                let tilemap = match state.tilemaps.get(&entity_id) {
                    Some(tm) => tm,
                    None => {
                        warn!(
                            entity_id,
                            "No tilemap found for entity when checking walkable"
                        );
                        return Err("No tilemap found for entity");
                    }
                };

                let walkable = tilemap.is_walkable(x, y);
                debug!(entity_id, x, y, walkable, "Checked tile walkable status");
                Ok(walkable)
            }
            Err(e) => {
                error!(
                    ?e,
                    entity_id, "Failed to borrow game state while checking walkable"
                );
                Err("Failed to borrow game state")
            }
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
                if !state.tilemaps.contains_key(&tilemap_id) {
                    warn!(
                        entity_id,
                        tilemap_id, "Tilemap not found for position check"
                    );
                    return Err("Tilemap not found");
                }

                debug!(entity_id, tilemap_id, x, y, "Checking position walkable");

                match collision_manager
                    .check_entity_tilemap_collision(entity_id, tilemap_id, x, y, assets)
                {
                    Ok(collision) => {
                        debug!(
                            entity_id,
                            tilemap_id,
                            x,
                            y,
                            collision,
                            walkable = !collision,
                            "Position walkable check complete"
                        );
                        Ok(!collision)
                    }
                    Err(e) => {
                        error!(?e, entity_id, tilemap_id, x, y, "Error checking collision");
                        Err(e)
                    }
                }
            }
            Err(e) => {
                error!(
                    ?e,
                    entity_id, "Failed to borrow game state while checking position"
                );
                Err("Failed to borrow game state")
            }
        }
    }

    pub fn get_tile_size(&self, entity_id: u32) -> Result<u32, &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                let tilemap = match state.tilemaps.get(&entity_id) {
                    Some(tm) => tm,
                    None => {
                        warn!(
                            entity_id,
                            "No tilemap found for entity when getting tile size"
                        );
                        return Err("No tilemap found for entity");
                    }
                };

                debug!(
                    entity_id,
                    tile_size = tilemap.tile_size,
                    "Retrieved tile size"
                );
                Ok(tilemap.tile_size)
            }
            Err(e) => {
                error!(
                    ?e,
                    entity_id, "Failed to borrow game state while getting tile size"
                );
                Err("Failed to borrow game state")
            }
        }
    }

    pub fn get_dimensions(&self, entity_id: u32) -> Result<(u32, u32), &'static str> {
        match self.state.try_borrow() {
            Ok(state) => {
                let tilemap = match state.tilemaps.get(&entity_id) {
                    Some(tm) => tm,
                    None => {
                        warn!(
                            entity_id,
                            "No tilemap found for entity when getting dimensions"
                        );
                        return Err("No tilemap found for entity");
                    }
                };

                debug!(
                    entity_id,
                    width = tilemap.width,
                    height = tilemap.height,
                    "Retrieved tilemap dimensions"
                );
                Ok((tilemap.width, tilemap.height))
            }
            Err(e) => {
                error!(
                    ?e,
                    entity_id, "Failed to borrow game state while getting dimensions"
                );
                Err("Failed to borrow game state")
            }
        }
    }
}

