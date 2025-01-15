# Input API

[Back to Index](index.md)

The Input API provides functions for handling player input from both keyboard and mouse. This API enables games to detect when specific keys or mouse buttons are pressed, allowing for responsive player controls.

## Keyboard Functions

### `is_key_pressed(key)`
Checks if a specific keyboard key is currently being pressed.

**Parameters**:
- `key` (string): The name of the key to check. Key names are case-insensitive.

**Returns**:
- `true` if the specified key is currently pressed
- `false` if the key is not pressed or the key name is invalid

**Supported Keys**:
- Letter keys: `"A"` through `"Z"`
- Arrow keys: `"LEFT"`, `"RIGHT"`, `"UP"`, `"DOWN"`
- Special keys: `"SPACE"`, `"RETURN"`, `"TAB"`

**Example**:
```lua
-- Check for player movement input
if is_key_pressed("W") then
    move_player_up()
end

if is_key_pressed("SPACE") then
    player_jump()
end
```

## Mouse Functions

### `is_mouse_pressed(button)`
Checks if a specific mouse button is currently being pressed.

**Parameters**:
- `button` (string): The mouse button to check. Button names are case-insensitive.

**Returns**:
- `true` if the specified button is currently pressed
- `false` if the button is not pressed or the button name is invalid

**Supported Buttons**:
- `"LEFT"`: Left mouse button
- `"RIGHT"`: Right mouse button
- `"MIDDLE"`: Middle mouse button (scroll wheel)

**Example**:
```lua
-- Check for mouse input
if is_mouse_pressed("LEFT") then
    shoot_projectile()
end

if is_mouse_pressed("RIGHT") then
    activate_shield()
end
```

## Notes
- All input functions are designed to work continuously while the key or button is held down
- Input names are case-insensitive for convenience
- Invalid input names will cause the functions to return false rather than error
- Mouse position is automatically tracked and available through the global variables `mouse_x` and `mouse_y`
