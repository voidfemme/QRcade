# Tilemap API

[Back to Index](index.md)

Tilemaps allow you to create and manage grid-based game worlds. Use the `tilemap_api` to create tile-based levels, manage terrain, and handle grid-based game logic.

## Functions

### `create_tilemap(entity_id, width, height, tile_size)`

Creates a new tilemap and associates it with the specified entity.

**Parameters**:
- `entity_id` (number): The entity to attach the tilemap to
- `width` (number): Width of the tilemap in tiles
- `height` (number): Height of the tilemap in tiles
- `tile_size` (number): Size of each tile in pixels

**Example**:
```lua
local map = create_entity()
create_tilemap(map, 10, 10, 32)  -- Creates a 10x10 tilemap with 32x32 pixel tiles
```

### `set_tile(entity_id, x, y, tile_id, walkable, r, g, b)`

Sets a tile at the specified position in the tilemap.

**Parameters**:
- `entity_id` (number): The entity with the tilemap
- `x` (number): X coordinate in the tilemap
- `y` (number): Y coordinate in the tilemap
- `tile_id` (number): Identifier for the tile type
- `walkable` (boolean): Whether entities can traverse this tile
- `r` (number): Red color component (0-255)
- `g` (number): Green color component (0-255)
- `b` (number): Blue color component (0-255)

**Example**:
```lua
-- Create a red wall tile
set_tile(map, 0, 0, 1, false, 255, 0, 0)  

-- Create a green walkable tile
set_tile(map, 1, 1, 2, true, 0, 255, 0)   
```

### `clear_tile(entity_id, x, y)`

Removes a tile at the specified position in the tilemap.

**Parameters**:
- `entity_id` (number): The entity with the tilemap
- `x` (number): X coordinate in the tilemap
- `y` (number): Y coordinate in the tilemap

**Example**:
```lua
clear_tile(map, 0, 0)  -- Removes the tile at position (0,0)
```

### `is_walkable(entity_id, x, y)`

Checks if a position in the tilemap can be traversed.

**Parameters**:
- `entity_id` (number): The entity with the tilemap
- `x` (number): X coordinate in the tilemap
- `y` (number): Y coordinate in the tilemap

**Returns**:
- `walkable` (boolean): Whether the position can be traversed

**Example**:
```lua
if is_walkable(map, player_x, player_y) then
    -- Move player to position
end
```

### `query_tilemap(entity_id, query_type, args)`

Retrieves information about the tilemap using various query types.

**Parameters**:
- `entity_id` (number): The entity with the tilemap
- `query_type` (string): Type of query to perform
- `args` (table): Query-specific arguments

**Query Types**:
- `"dimensions"`: Returns tilemap dimensions
  - No additional arguments needed
- `"tile"`: Returns data for a specific tile
  - `args.x` (number): X coordinate
  - `args.y` (number): Y coordinate
- `"area"`: Returns data for a rectangular area of tiles
  - `args.x` (number): Starting X coordinate
  - `args.y` (number): Starting Y coordinate
  - `args.width` (number): Width of area
  - `args.height` (number): Height of area
- `"row"`: Returns data for an entire row
  - `args.y` (number): Y coordinate of row
- `"column"`: Returns data for an entire column
  - `args.x` (number): X coordinate of column

**Examples**:
```lua
-- Get tilemap dimensions
local dims = query_tilemap(map, "dimensions", {})
print("Map size: " .. dims.width .. "x" .. dims.height)

-- Get a specific tile
local tile = query_tilemap(map, "tile", {x = 1, y = 1})
if tile then
    print("Tile ID: " .. tile.tile_id)
end

-- Get a 2x2 area of tiles
local area = query_tilemap(map, "area", {x = 0, y = 0, width = 2, height = 2})
for _, tile in ipairs(area) do
    print("Tile at " .. tile.x .. "," .. tile.y)
end

-- Get entire row
local row = query_tilemap(map, "row", {y = 0})
for _, tile in ipairs(row) do
    print("Tile at column " .. tile.position)
end
```
