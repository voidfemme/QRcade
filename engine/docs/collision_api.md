# Collision API
The `collision_api` checks for interactions between entities.

## Functions
### `is_colliding(entity_1, entity_2)`
Determines if two entities are colliding.
**Parameters**:
- `entity_1`, `entity_2` (number): The IDs of the entities to check.
**Returns**:
- `true` if the entities are colliding, `false` otherwise.
**Example**:
```
if is_colliding(player, enemy) then
  reduce_health(player, 10)
end
```
