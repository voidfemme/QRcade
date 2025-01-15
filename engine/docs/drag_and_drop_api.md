# Drag and Drop API

[Back to Index](index.md)

The Drag and Drop API provides functions for implementing drag-and-drop interactions with game entities. This API enables games to create interactive objects that players can click and drag around the game world.

## Functions

### `can_drag_entity(x, y)`
Checks if there is a draggable entity at the specified coordinates.

**Parameters**:
- `x` (number): The x-coordinate to check
- `y` (number): The y-coordinate to check

**Returns**:
- If an entity is found: returns the entity ID (number)
- If no entity is found: returns nil

**Example**:
```lua
local entity = can_drag_entity(mouse_x, mouse_y)
if entity then
    print("Found draggable entity: " .. entity)
end
```

### `start_drag(entity_id, x, y)`
Initiates dragging of a specified entity from the given coordinates.

**Parameters**:
- `entity_id` (number): The ID of the entity to drag
- `x` (number): The x-coordinate where dragging begins
- `y` (number): The y-coordinate where dragging begins

**Returns**:
- `true` if dragging started successfully
- Throws an error if the entity cannot be dragged

**Example**:
```lua
if is_mouse_pressed("LEFT") then
    local entity = can_drag_entity(mouse_x, mouse_y)
    if entity then
        start_drag(entity, mouse_x, mouse_y)
    end
end
```

### `update_drag(x, y)`
Updates the position of the currently dragged entity.

**Parameters**:
- `x` (number): The new x-coordinate for the dragged entity
- `y` (number): The new y-coordinate for the dragged entity

**Returns**:
- Nothing on success
- Throws an error if no entity is being dragged or if the update fails

**Example**:
```lua
if is_dragging(entity_id) then
    update_drag(mouse_x, mouse_y)
end
```

### `end_drag()`
Ends the current drag operation.

**Parameters**:
- None

**Returns**:
- Nothing on success
- Throws an error if ending the drag operation fails

**Example**:
```lua
if not is_mouse_pressed("LEFT") then
    end_drag()
end
```

### `is_dragging(entity_id)`
Checks if a specific entity is currently being dragged.

**Parameters**:
- `entity_id` (number): The ID of the entity to check

**Returns**:
- `true` if the specified entity is being dragged
- `false` otherwise

**Example**:
```lua
if is_dragging(player_entity) then
    show_drag_effect()
end
```

## Complete Example
Here's a complete example showing how to implement drag and drop functionality:

```lua
function on_frame(delta_time)
    if is_mouse_pressed("LEFT") then
        -- If we're not already dragging something
        if not is_dragging(entity_id) then
            -- Check if we can start dragging
            local entity = can_drag_entity(mouse_x, mouse_y)
            if entity then
                start_drag(entity, mouse_x, mouse_y)
            end
        else
            -- Update the position of what we're dragging
            update_drag(mouse_x, mouse_y)
        end
    else
        -- Mouse released, end any drag operation
        end_drag()
    end
end
```

## Notes
- Only one entity can be dragged at a time
- Dragging automatically maintains the initial offset between the mouse cursor and the entity's position
- The drag state is automatically cleared when the left mouse button is released
- All coordinates are in world space
- Error handling should be implemented for robust drag and drop functionality
