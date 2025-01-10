-- Create two rectangles: one player-controlled and one stationary
function on_start()
    -- Player rectangle (red) - controlled by arrow keys
    local player = create_entity()  
    set_transform(player, 100, 300, 0, 1.0, 1.0)
    add_rectangle(player, 50, 50, 255, 0, 0)  -- Red square

    -- Stationary rectangle (blue) - acts as a "goal"
    local goal = create_entity()
    set_transform(goal, 500, 300, 0, 1.0, 1.0)
    add_rectangle(goal, 60, 60, 0, 0, 255)  -- Blue square
end

-- Handle movement and collision checking every frame
function on_frame(delta_time)
    local player = 1  -- First entity we created
    local goal = 2    -- Second entity we created
    
    -- Get player position
    local x, y, rotation = get_transform(player)
    
    -- Move player with arrow keys
    local speed = 200  -- Pixels per second
    if is_key_pressed("RIGHT") then
        x = x + speed * delta_time
    end
    if is_key_pressed("LEFT") then
        x = x - speed * delta_time
    end
    if is_key_pressed("UP") then
        y = y - speed * delta_time
    end
    if is_key_pressed("DOWN") then
        y = y + speed * delta_time
    end
    
    -- Update player position
    set_transform(player, x, y, rotation, 1.0, 1.0)
    
    -- Check for collision and change player color
    if is_colliding(player, goal) then
        -- Turn player green when colliding
        add_rectangle(player, 50, 50, 0, 255, 0)
        print("Collision detected!")
    else
        -- Return to red when not colliding
        add_rectangle(player, 50, 50, 255, 0, 0)
    end
end

function on_end()
    print("Game ended!")
end
