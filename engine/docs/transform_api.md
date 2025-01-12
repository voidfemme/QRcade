# Transform API

[Back to Index](index.md)

The `transform_api` allows you to control the position, rotation and scale of entities.

## Functions

### `set_transform(entity_id, x, y, rotation, scale_x, scale_y)`

Sets the transform properties for an entity 

**Parameters**
- `entity_id` (number): The ID of the entity.
- `x`, `y` (number): Position coordinates.
- `rotation` (number): Rotation angle in degrees.
- `scale_x`, `scale_y` (number): Scale factors for width and height

**Example**
```lua
set_transform(player, 100, 200, 0, 1.0, 1.0)
```

### `get_transform(entity_id)`

Retrieves the transform properties of an entity.

**Returns**: 
- `(x, y, rotation, scale_x, scale_y)`

**Example**:
```lua
local x, y, rotation, scale_x, scale_y = get_transform(player)
```


