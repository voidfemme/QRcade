# QRcade Game Engine Documentation

Welcome to the QRcade Game Engine documentation! Whether you're just starting or a seasoned developer, this guide will help you leverage the engine's powerful features to build your game. Below is a categorized overview of the engine's capabilities, along with links to detailed API documentation and guides.

## Getting Started

To begin coding, follow these steps:

1. **Setup Your Game:** Define the main entry points for your game logic using [Script Callback Functions](script_callbacks.md).
2. **Create Game Entities:** Use the [Entity API](entity_api.md) to create the building blocks of your game world.
3. **Design Your Game World:** Utilize the [Tilemap API](tilemap_api.md) for grid-based levels, and the [Transform API](transform_api.md) to position and scale your entities.
4. **Add Movement:** Control entity motion with the [Velocity API](velocity_api.md).
5. **Handle Input:** Use the [Input API](input_api.md) to make your game interactive.
6. **Render Visuals:** Bring your entities to life with the [Renderable API](renderable_api.md).
7. **Implement Interactions:** Detect and manage interactions between entities using the [Collision API](collision_api.md).

## Documentation Overview

### Core Components
- **[Entity API](entity_api.md):** Manage entities, the fundamental objects in your game world.
- **[Transform API](transform_api.md):** Adjust position, rotation, and scale for entities.
- **[Velocity API](velocity_api.md):** Implement movement and control dynamics.

### Gameplay and Logic
- **[Tilemap API](tilemap_api.md):** Create grid-based worlds and manage terrain.
- **[Script Callback Functions](script_callbacks.md):** Lifecycle functions for game initialization, updates, and cleanup.
- **[Collision API](collision_api.md):** Handle entity collisions and interactions.

### Input and Rendering
- **[Input API](input_api.md):** Capture player input from the keyboard.
- **[Renderable API](renderable_api.md):** Add and configure visual elements for entities.

## Examples

### Setting Up a Basic Game
Start with `on_start()` to initialize your game world:
```lua
function on_start()
  local player = create_entity()
  set_transform(player, 50, 50, 0, 1, 1)
  add_shape(player, "circle", 255, 0, 0, {radius = 20})
end
```

### Making an Entity Move
Use the Velocity and Input APIs together:
```lua
function on_frame(delta_time)
  if is_key_pressed("LEFT") then
    set_velocity(player, -100, 0)
  elseif is_key_pressed("RIGHT") then
    set_velocity(player, 100, 0)
  else
    set_velocity(player, 0, 0)
  end
end
```

### Managing Collisions
Detect and handle collisions between entities:
```lua
if is_colliding(player, enemy) then
  print("Collision detected!")
end
```

## Need Help?

If you're stuck or looking for deeper insights, refer to the respective API documentation or explore the provided examples in each file.

---

Happy coding and enjoy building w
