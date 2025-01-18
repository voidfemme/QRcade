# Text API

[Back to Index](index.md)

The Text API provides functions for adding and manipulating text components in game entities. This API enables games to display and update text for scores, messages, UI elements, and other text-based game content.

## Constants

### Text IDs
Available through the `TEXT` table:
- `TEXT.GAME_OVER`: "Game Over"
- `TEXT.PRESS_SPACE`: "Press Space"
- `TEXT.SCORE`: "Score: "
- `TEXT.YOU_WIN`: "You win!"
- `TEXT.GAME_PAUSED`: "Game Paused"
- `TEXT.PRESS_ESC_TO_QUIT`: "Press ESC to Quit"
- `TEXT.LIVES`: "Lives: "
- `TEXT.LEVEL`: "Level: "
- `TEXT.READY`: "Ready"
- `TEXT.GO`: "Go!"
- `TEXT.HIGH_SCORE`: "High Score: "
- `TEXT.NEW_HIGH_SCORE`: "New High Score!"
- `TEXT.TRY_AGAIN`: "Press Space to Try Again"

### Alignment Constants
Available through the `ALIGN` table:

Horizontal Alignment:
- `ALIGN.LEFT`
- `ALIGN.CENTER`
- `ALIGN.RIGHT`

Vertical Alignment:
- `ALIGN.TOP`
- `ALIGN.MIDDLE`
- `ALIGN.BOTTOM`

## Functions

### `add_text(entity_id, text_id, params)`
Adds a text component to an entity with specified text and optional parameters.

**Parameters**:
- `entity_id` (number): The ID of the entity to add text to
- `text_id` (TEXT constant): The type of text to display
- `params` (table, optional): Configuration options:
  - `color` (table): RGB values {r, g, b}
  - `scale` (number): Text scale factor
  - `h_align` (ALIGN constant): Horizontal alignment
  - `v_align` (ALIGN constant): Vertical alignment

**Returns**:
- Nothing on success
- Throws an error if the entity doesn't exist or if adding text fails

**Example**:
```lua
local score_display = create_entity()
add_text(score_display, TEXT.SCORE, {
    color = {255, 255, 255},
    scale = 2.0,
    h_align = ALIGN.CENTER,
    v_align = ALIGN.TOP
})
```

### `update_text(entity_id, text_id)`
Updates the text content of an existing text component.

**Parameters**:
- `entity_id` (number): The ID of the entity with the text component
- `text_id` (TEXT constant): The new text to display

**Returns**:
- Nothing on success
- Throws an error if the entity has no text component

**Example**:
```lua
-- Update score display when player scores
if player_scored then
    update_text(score_display, TEXT.SCORE)
end
```

### `set_text_color(entity_id, r, g, b)`
Sets the color of an entity's text component.

**Parameters**:
- `entity_id` (number): The ID of the entity with the text component
- `r` (number): Red value (0-255)
- `g` (number): Green value (0-255)
- `b` (number): Blue value (0-255)

**Returns**:
- Nothing on success
- Throws an error if the entity has no text component

**Example**:
```lua
-- Make text red when player is low on health
if health < 20 then
    set_text_color(health_display, 255, 0, 0)
end
```

### `set_text_scale(entity_id, scale)`
Sets the scale of an entity's text component.

**Parameters**:
- `entity_id` (number): The ID of the entity with the text component
- `scale` (number): Scale factor for the text

**Returns**:
- Nothing on success
- Throws an error if the entity has no text component

**Example**:
```lua
-- Make text bigger when highlighting
set_text_scale(menu_item, 1.5)
```

### `set_text_visibility(entity_id, visible)`
Sets whether an entity's text component is visible.

**Parameters**:
- `entity_id` (number): The ID of the entity with the text component
- `visible` (boolean): Whether the text should be visible

**Returns**:
- Nothing on success
- Throws an error if the entity has no text component

**Example**:
```lua
-- Hide text when game is paused
set_text_visibility(gameplay_text, not is_paused)
```

## Complete Example
Here's a complete example showing how to create and manage text components:

```lua
-- Create score display
local score_display = create_entity()
set_transform(score_display, 10, 10, 0)
add_text(score_display, TEXT.SCORE, {
    color = {255, 255, 255},
    scale = 1.0,
    h_align = ALIGN.LEFT,
    v_align = ALIGN.TOP
})

-- Create game over text (initially hidden)
local game_over_text = create_entity()
set_transform(game_over_text, 400, 300, 0)
add_text(game_over_text, TEXT.GAME_OVER, {
    color = {255, 0, 0},
    scale = 3.0,
    h_align = ALIGN.CENTER,
    v_align = ALIGN.MIDDLE
})
set_text_visibility(game_over_text, false)

function on_frame(delta_time)
    -- Update score display
    if score_changed then
        update_text(score_display, TEXT.SCORE)
    end
    
    -- Show game over when player dies
    if player_health <= 0 then
        set_text_visibility(game_over_text, true)
    end
end
```

## Notes
- Text components are rendered based on their entity's transform position
- Text scale affects both the size and positioning of the text
- Text visibility can be toggled without removing the text component
- Color values should be in the range 0-255
- Alignment affects how text is positioned relative to its entity's transform
- The available text constants are fixed and cannot be modified at runtime
