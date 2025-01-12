# Input API

[Back to Index](index.md)

The `input_api` handles player input from the keyboard

## Functions:
###`is_key_pressed(key)`
Checks if a specific key is currently pressed.

**Parameters**:
- `key` (string): The key name (e.g., `"A"`, `"SPACE"`, `"LEFT"`).

**Returns**:
- `true` if the key is pressed, `false` otherwise.

**Example**:
``` lua
if is_key_pressed("SPACE") then
  jump()
end
```
