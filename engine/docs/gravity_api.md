# Gravity API

[Back to Index](index.md)

The Gravity API allows you to add different types of gravity effects to entities. This includes traditional downward gravity, attractive forces (like planets), and repulsive forces (like explosions).

## Functions

### `add_downward_gravity(entity_id, force, terminal_velocity)`

Adds traditional downward gravity to an entity.

**Parameters**:
- `entity_id` (number): The entity to add gravity to
- `force` (number): The strength of gravity (positive numbers pull downward)
- `terminal_velocity` (number): Maximum falling speed

**Example**:
```lua
local player = create_entity()
add_downward_gravity(player, 1, 10)  -- Normal downward gravity
```

### `add_attractive_gravity(entity_id, force, terminal_velocity)`

Adds attractive gravity to an entity, making it pull other entities toward it like a planet or magnet.

**Parameters**:
- `entity_id` (number): The entity to make attractive
- `force` (number): The strength of attraction (higher numbers create stronger pull)
- `terminal_velocity` (number): Maximum speed entities can be pulled

**Example**:
```lua
local planet = create_entity()
add_attractive_gravity(planet, 5000, 200)  -- Strong attractive force
```

### `add_repulsive_gravity(entity_id, force, terminal_velocity)`

Adds repulsive gravity to an entity, making it push other entities away like an explosion.

**Parameters**:
- `entity_id` (number): The entity to make repulsive
- `force` (number): The strength of repulsion (higher numbers push harder)
- `terminal_velocity` (number): Maximum speed entities can be pushed

**Example**:
```lua
local explosion = create_entity()
add_repulsive_gravity(explosion, 1000, 300)  -- Strong repulsive force
```

### `set_gravity_enabled(entity_id, enabled)`

Enables or disables gravity for an entity that already has a gravity component.

**Parameters**:
- `entity_id` (number): The entity to modify
- `enabled` (boolean): Whether gravity should be enabled (true) or disabled (false)

**Example**:
```lua
-- Temporarily disable gravity
set_gravity_enabled(player, false)

-- Re-enable gravity
set_gravity_enabled(player, true)
```

## Notes
- Attractive and repulsive gravity affects all other entities with velocity components
- Force follows an inverse square law (gets weaker with distance)
- Terminal velocity applies to both horizontal and vertical movement for attractive/repulsive gravity
- For downward gravity, terminal velocity only applies to vertical movement
