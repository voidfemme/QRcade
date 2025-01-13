-- Create player entity
local player = create_entity()

-- Set initial position at center of screen
local pos = {
    x = 400,  -- middle of 800 width
    y = 300   -- middle of 600 height
}

-- Create a green square for the player
set_transform(player, pos.x, pos.y, 0, 1.0, 1.0)
add_shape(player, "square", 0, 255, 0, {width = 32, height = 32})

-- Movement speed (pixels per second)
local SPEED = 200

function on_frame(delta_time)
    -- Get input and update position
    if is_key_pressed("UP") or is_key_pressed("W") then
        pos.y = pos.y - SPEED * delta_time
    end
    if is_key_pressed("DOWN") or is_key_pressed("S") then
        pos.y = pos.y + SPEED * delta_time
    end
    if is_key_pressed("LEFT") or is_key_pressed("A") then
        pos.x = pos.x - SPEED * delta_time
    end
    if is_key_pressed("RIGHT") or is_key_pressed("D") then
        pos.x = pos.x + SPEED * delta_time
    end

    -- Update transform with new position
    set_transform(player, pos.x, pos.y, 0, 1.0, 1.0)
end

print("Use WASD or arrow keys to move the green square")
