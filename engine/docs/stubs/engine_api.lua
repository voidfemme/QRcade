---@diagnostic disable: lowercase-global

-- Velocity API
--- Sets the velocity of an entity.
--- @param entity_id number The ID of the entity
--- @param velocity_x number The X component of the velocity
--- @param velocity_y number The Y component of the velocity
function set_velocity(entity_id, velocity_x, velocity_y) end

--- Gets the velocity of an entity.
--- @param entity_id number The ID of the entity
--- @return table velocity {x = number, y = number} The velocity components
function get_velocity(entity_id)
	return { x = 0, y = 0 } -- Default return value
end

-- Transform API
--- Sets the transform (position, rotation, scale) of an entity.
--- @param entity_id number The ID of the entity
--- @param x number The X position
--- @param y number The Y position
--- @param rotation number The rotation (in degrees)
--- @param scale_x number The X scale
--- @param scale_y number The Y scale
function set_transform(entity_id, x, y, rotation, scale_x, scale_y) end

--- Gets the transform of an entity.
--- @param entity_id number The ID of the entity
--- @return table transform {x = number, y = number, rotation = number, scale_x = number, scale_y = number}
function get_transform(entity_id)
	return { x = 0, y = 0, rotation = 0, scale_x = 1, scale_y = 1 } -- Default return value
end

-- Tilemap API
--- Creates a tilemap for an entity.
--- @param entity_id number The ID of the entity
--- @param width number The width of the tilemap in tiles
--- @param height number The height of the tilemap in tiles
--- @param tile_size number The size of each tile
function create_tilemap(entity_id, width, height, tile_size) end

--- Sets a tile in a tilemap.
--- @param entity_id number The ID of the entity
--- @param x number The X position of the tile
--- @param y number The Y position of the tile
--- @param tile_id number The ID of the tile
--- @param walkable boolean Whether the tile is walkable
--- @param r number Red color value (0-255)
--- @param g number Green color value (0-255)
--- @param b number Blue color value (0-255)
function set_tile(entity_id, x, y, tile_id, walkable, r, g, b) end

--- Clears a tile in a tilemap.
--- @param entity_id number The ID of the entity
--- @param x number The X position of the tile
--- @param y number The Y position of the tile
function clear_tile(entity_id, x, y) end

--- Queries a tilemap for data.
--- @param entity_id number The ID of the entity
--- @param query_type string The type of query ("dimensions", "tile", "area", "row", "column")
--- @param args table Arguments for the query
--- @return table Query results depending on the query type
function query_tilemap(entity_id, query_type, args)
	return {} -- Default return value
end

--- Checks if a tile position is walkable.
--- @param entity_id number The ID of the entity
--- @param x number The X position
--- @param y number The Y position
--- @return boolean Whether the position is walkable
function is_walkable(entity_id, x, y)
	return false -- Default return value
end

-- Renderable API
--- Adds a shape to an entity for rendering.
--- @param entity_id number The ID of the entity
--- @param shape_name string The name of the shape ("square", "circle", "triangle", "line")
--- @param r number Red color value (0-255)
--- @param g number Green color value (0-255)
--- @param b number Blue color value (0-255)
--- @param params table Optional parameters for the shape
function add_shape(entity_id, shape_name, r, g, b, params) end

-- Input API
--- Checks if a key is currently pressed.
--- @param key string The name of the key (e.g., "A", "LEFT", "SPACE")
--- @return boolean Whether the key is pressed
function is_key_pressed(key)
	return false -- Default return value
end

-- Entity API
--- Creates a new entity.
--- @return number entity_id The ID of the newly created entity
function create_entity()
	return 0 -- Default return value
end

--- Destroys an entity.
--- @param entity_id number The ID of the entity to destroy
function destroy_entity(entity_id) end

-- Collision API
--- Checks if two entities are colliding.
--- @param entity1 number The ID of the first entity
--- @param entity2 number The ID of the second entity
--- @return boolean Whether the entities are colliding
function is_colliding(entity1, entity2)
	return false -- Default return value
end
