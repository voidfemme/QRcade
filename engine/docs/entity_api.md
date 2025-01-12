# Entity API

Entities are the fundamental building blocks of the game world. Use the `entity_api` to create and manage them.

## **Functions**

### `create_entity()`

Creates a new entity in the game world.

**Returns**: 
- `entity_id` (unique identifier for the entity)

**Example**:
```
local player = create_entity()
```

### `destroy_entity()`

Removes the specified entity from the game world

**Parameters**: 
- `entity_id` (number): The ID of the entity to remove

**Example**:
```
destroy_entity(player)
```
