# Script Callback Functions
[Back to Index](index.md)

The QRcade Game Engine looks for specific callback functions in your Lua scripts.
Defining these functions allows the engine to properly initialize, update, and finalize
your game logic. The engine automatically calls these functions at the appropriate
times during the game lifecycle if they are defined.

## `on_start()`
Called once after the Lua script is loaded, during the initialization phase.

**Usage**:

Use this function to set up game entities, initialize tilemaps, load assets, or 
configure game settings.

**Example**:
```lua
function on_start()
  -- Initialize game world, entities, etc.
  print("Game is starting!")
end
```

## `on_frame()`
Called once per frame with the elapsed time (`delta_time`) since the last frame.

**Usage**:

Use this function for per-frame game logic, such as handling input, moving entities,
checking collisions, and updating the game state.

**Example**:
```lua
function on_frame(delta_time)
  -- Handle per-frame logic, e.g., player movement
  if is_key_pressed("W") then
    move_player(0, -1)
  end
  -- Add similar checks for "A", "S", "D" keys...
end
```
**Note**:

Replace any `update()` function with `on_frame(delta_time)` in your scripts for compatibility
with the engine's callback mechanism.

## `on_end()`
Called once when the game is exiting or when the Lua environment is closing.

**Usage**:

Use this function to perform cleanup tasks, save game state, or release resources.

**Example**:
```lua
function on_end()
  -- Initialize game world, entities, etc.
  print("Game is ending.")
end
```
