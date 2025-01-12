# Tilemap API

[Back to Index](index.md)

Tilemaps allow you to create and manage grid-based game worlds. Use the 
`tilemap_api` to create tile-based levels, manage terrain, and handle 
grid-based game logic.

## Functions

### `create_tilemap(entity_id, width, height, tile_size)`
Creates a new tilemap and associates it with the specified entity.

**Parameters**:
- `entity_id` (number): The entity to attach the tilemap to
- `width` (number): Width of the tilemap in tiles 
- `height` (number): Height of the tilemap in tiles
- `tile_size` (number): Size of each tile in pixels

**Example**:
```
local map = create_entity()
create_tilemap(map, 10, 10, 32) -- Creates a 10x10 tilemap with 32x32 pixel tiles
```
