# QRcade Game Engine Documentation
Welcome to the QRcade Game Engine documentation! This comprehensive guide will help you harness the engine's capabilities to create engaging interactive games. Whether you're building a simple puzzle game or a complex physics-based adventure, you'll find everything you need to bring your vision to life.

## Getting Started
To begin creating your game, follow these fundamental steps:
1. **Setup Your Game:** Define the main entry points for your game logic using [Script Callback Functions](script_callbacks.md).
2. **Create Game Entities:** Use the [Entity API](entity_api.md) to create the building blocks of your game world.
3. **Design Your Game World:** Utilize the [Tilemap API](tilemap_api.md) for grid-based levels, and the [Transform API](transform_api.md) to position and scale your entities.
4. **Add Movement:** Control entity motion with the [Velocity API](velocity_api.md) and add physics with the [Gravity API](gravity_api.md).
5. **Handle Input:** Make your game interactive using the [Input API](input_api.md).
6. **Implement Interactions:** Add drag-and-drop functionality with the [Drag and Drop API](drag_drop_api.md).
7. **Render Visuals:** Bring your entities to life with the [Renderable API](renderable_api.md) and add text with the [Text API](text_api.md).
8. **Manage Collisions:** Detect and handle interactions between entities using the [Collision API](collision_api.md).

## Documentation Overview

### Core Components
- **[Entity API](entity_api.md):** Create and manage entities, the fundamental objects in your game world.
- **[Transform API](transform_api.md):** Control position, rotation, and scale for precise entity placement.
- **[Velocity API](velocity_api.md):** Implement smooth movement and physics-based motion.
- **[Gravity API](gravity_api.md):** Add realistic physics with customizable gravity effects.
- **[Timer API](timer_api.md):** Provides tools to create and manage timed events and
  intervals in your game.

### Interaction and Input
- **[Input API](input_api.md):** Handle keyboard and mouse input for player interaction.
- **[Drag and Drop API](drag_drop_api.md):** Create interactive objects players can click and drag.
- **[Collision API](collision_api.md):** Detect and respond to entity collisions and interactions.

### World Building
- **[Tilemap API](tilemap_api.md):** Design levels with grid-based terrain and obstacles.
- **[Renderable API](renderable_api.md):** Configure visual elements and appearance.
- **[Text API](text_api.md):** Add and manage text for UI, scores, and messages.
- **[Script Callback Functions](script_callbacks.md):** Manage game initialization, updates, and cleanup.

## Code Examples

### Basic Game Setup
Initialize your game world with a draggable object and score display:
```lua
function on_start()
    -- Create a draggable ball
    local ball = create_entity()
    set_transform(ball, 400, 300, 0, 1, 1)
    add_shape(ball, "circle", 0, 255, 0, {radius = 20})
    
    -- Add score display
    local score = create_entity()
    set_transform(score, 10, 10, 0)
    add_text(score, TEXT.SCORE, {
        color = {255, 255, 255},
        scale = 1.5,
        h_align = ALIGN.LEFT,
        v_align = ALIGN.TOP
    })
    
    print("Click and drag the green ball!")
end
```

### Implementing Drag and Drop
Create interactive objects players can move:
```lua
function on_frame(delta_time)
    if is_mouse_pressed("LEFT") then
        if not is_dragging(ball) then
            -- Check if we can start dragging
            local entity = can_drag_entity(mouse_x, mouse_y)
            if entity then
                start_drag(entity, mouse_x, mouse_y)
            end
        else
            -- Update dragged object position
            update_drag(mouse_x, mouse_y)
        end
    else
        -- Release when mouse button is up
        end_drag()
    end
end
```

### Physics and Movement
Combine gravity with draggable objects:
```lua
-- Create a planet with attractive gravity
local planet = create_entity()
set_transform(planet, 400, 300, 0, 1, 1)
add_shape(planet, "circle", 255, 255, 0, {radius = 32})
add_attractive_gravity(planet, 5000, 200)

-- Create draggable satellites
local satellite = create_entity()
set_transform(satellite, 500, 300, 0, 1, 1)
add_shape(satellite, "circle", 0, 255, 255, {radius = 16})
```

### Advanced Interactions
Combine dragging with collision detection and text feedback:
```lua
function on_frame(delta_time)
    if is_dragging(satellite) then
        -- Check for collision with planet while dragging
        if is_colliding(satellite, planet) then
            -- Show collision message
            local message = create_entity()
            set_transform(message, 400, 200, 0)
            add_text(message, TEXT.GAME_OVER, {
                color = {255, 0, 0},
                scale = 2.0,
                h_align = ALIGN.CENTER,
                v_align = ALIGN.MIDDLE
            })
            
            -- Handle collision effects
            set_zero_velocity(satellite)
            end_drag()
        end
    end
end
```

## Feature Integration Tips

### Combining Drag and Drop with Physics
When implementing draggable objects in a physics-based game:
1. Consider disabling gravity while objects are being dragged
2. Smoothly transition between dragged and physics-controlled states
3. Use collision detection to prevent dragging through solid objects

### Creating Interactive UI Elements
Build engaging user interfaces using drag and drop and text:
1. Create draggable buttons and controls with text labels
2. Implement drag-to-scroll functionality with dynamic text updates
3. Design drag-and-drop inventory systems with item descriptions
4. Add visual feedback through color-changing text

### Optimizing Performance
Keep your game running smoothly:
1. Limit the number of draggable objects on screen
2. Use appropriate collision shapes for drag detection
3. Implement efficient entity management
4. Update text content only when necessary

## Need Help?
If you need assistance or want to explore advanced features:
- Check the specific API documentation for detailed function references
- Review the example code provided in each section
- Experiment with combining different APIs for unique gameplay mechanics

---
Happy game development with QRcade!
