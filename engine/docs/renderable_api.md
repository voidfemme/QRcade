# Renderable API

[Back to Index](index.md)

The `renderable_api` manages visual representations of entities.

## Functions

### `add_shape(entity_id, shape_name, r, g, b, params)`

Adds a shape to an entity for rendering.

**Parameters**
- `entity_id` (number): The ID of the entity.
- `shape_name` (string): The type of shape:
  - "square", "circle", "triangle", "line"
- `r`, `g`, `b` (number): RGB color values (0-255).
- `params` (optional table): Additional parameters for the shape (e.g., size).

**Example**
```
add_shape(player, "triangle", 0, 255, 0, {x1 = 0, y1 = -16, x2 = -16, y2 = 16, x3 = 16, y3 = 16})
```
