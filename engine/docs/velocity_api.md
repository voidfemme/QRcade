# Velocity API

[Back to Index](index.md)

The `velocity_api` provides movement functionality for entities.

## Functions

### `set_velocity(entity_id, velocity_x, velocity_y)`

Sets the velocity of an entity 

**Parameters**:
- `entity_id` (number): The ID of the entity.
- `velocity_x`, `velocity_y` (number): Velocity components

**Example**:
```lua
set_velocity(player, 100, 0)
```

### `get_velocity(entity_id)`

Retrieves the velocity of an entity 

**Returns**:
- `velocity_x`, `velocity_y`

**Example**:
```lua
local vx, vy = get_velocity(player)
```
