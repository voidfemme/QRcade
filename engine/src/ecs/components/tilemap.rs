#[derive(Debug, Clone)]
pub struct TilemapView {
    pub width: u32,
    pub height: u32,
    pub tile_size: u32,
}

#[derive(Debug)]
pub struct TileView {
    pub tile_id: u32,
    pub walkable: bool,
    pub color: (u8, u8, u8),
}

#[derive(Debug)]
pub enum TilemapQuery {
    Dimensions,
    TileAt(u32, u32),
    Area {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    Row(u32),
    Column(u32),
}

pub enum TilemapQueryResult {
    Dimensions(TilemapView),
    Tile(Option<TileView>),
    Area(Vec<(u32, u32, TileView)>),
    Row(Vec<(u32, TileView)>),
    Column(Vec<(u32, TileView)>),
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub tile_id: u32,
    pub walkable: bool,
    pub color: (u8, u8, u8),
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            tile_id: 0,
            walkable: true,
            color: (255, 255, 255),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tilemap {
    pub width: u32,
    pub height: u32,
    pub tile_size: u32,
    pub tiles: Vec<Option<Tile>>,
}

impl Tilemap {
    pub fn new(width: u32, height: u32, tile_size: u32) -> Self {
        let tiles = vec![None; (width * height) as usize];
        Self {
            width,
            height,
            tile_size,
            tiles,
        }
    }

    pub fn query(&self, query: TilemapQuery) -> Result<TilemapQueryResult, &'static str> {
        match query {
            TilemapQuery::Dimensions => Ok(TilemapQueryResult::Dimensions(TilemapView {
                width: self.width,
                height: self.height,
                tile_size: self.tile_size,
            })),
            TilemapQuery::TileAt(x, y) => {
                let tile = self.get_tile(x, y).map(|t| TileView {
                    tile_id: t.tile_id,
                    walkable: t.walkable,
                    color: t.color,
                });
                Ok(TilemapQueryResult::Tile(tile))
            }
            TilemapQuery::Area {
                x,
                y,
                width,
                height,
            } => {
                let mut tiles = Vec::new();
                for cy in y..y + height {
                    for cx in x..x + width {
                        if let Some(tile) = self.get_tile(cx, cy) {
                            tiles.push((
                                cx,
                                cy,
                                TileView {
                                    tile_id: tile.tile_id,
                                    walkable: tile.walkable,
                                    color: tile.color,
                                },
                            ));
                        }
                    }
                }
                Ok(TilemapQueryResult::Area(tiles))
            }
            TilemapQuery::Row(y) => {
                let mut tiles = Vec::new();
                for x in 0..self.width {
                    if let Some(tile) = self.get_tile(x, y) {
                        tiles.push((
                            x,
                            TileView {
                                tile_id: tile.tile_id,
                                walkable: tile.walkable,
                                color: tile.color,
                            },
                        ));
                    }
                }
                Ok(TilemapQueryResult::Row(tiles))
            }
            TilemapQuery::Column(x) => {
                let mut tiles = Vec::new();
                for y in 0..self.height {
                    if let Some(tile) = self.get_tile(x, y) {
                        tiles.push((
                            y,
                            TileView {
                                tile_id: tile.tile_id,
                                walkable: tile.walkable,
                                color: tile.color,
                            },
                        ));
                    }
                }
                Ok(TilemapQueryResult::Column(tiles))
            }
        }
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Option<&Tile> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) as usize;
        self.tiles[index].as_ref()
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile: Tile) -> Result<(), &'static str> {
        if x >= self.width || y >= self.height {
            return Err("Tile position out of bounds");
        }
        let index = (y * self.width + x) as usize;
        self.tiles[index] = Some(tile);
        Ok(())
    }

    pub fn clear_tile(&mut self, x: u32, y: u32) -> Result<(), &'static str> {
        if x >= self.width || y >= self.height {
            return Err("Tile position out of bounds");
        }
        let index = (y * self.width + x) as usize;
        self.tiles[index] = None;
        Ok(())
    }

    pub fn is_walkable(&self, x: u32, y: u32) -> bool {
        self.get_tile(x, y).map_or(true, |tile| tile.walkable)
    }
}
