-- Create two entities with different colors and behaviors
function on_start()
    -- Create a red rectangle that moves horizontally
    local red = create_entity()  
    set_transform(red, 200, 300, 0, 1.0, 1.0)
    add_rectangle(red, 50, 50, 255, 0, 0)

    -- Create a blue rectangle that moves in a circle
    local blue = create_entity()
    set_transform(blue, 400, 300, 0, 1.0, 1.0)
    add_rectangle(blue, 30, 30, 0, 0, 255)
end

-- Move the entities every frame
function on_frame(delta_time)
    -- Move red rectangle left to right
    local red = 1
    local x, y, rotation = get_transform(red)
    set_transform(red, x + 100 * delta_time, y, rotation, 1.0, 1.0)

    -- Move blue rectangle in a circle
    local blue = 2
    local bx, by, brot = get_transform(blue)
    local radius = 100
    local angular_speed = 2  -- radians per second
    local angle = (brot + angular_speed * delta_time) % (2 * math.pi)
    local circle_x = 400 + radius * math.cos(angle)
    local circle_y = 300 + radius * math.sin(angle)
    set_transform(blue, circle_x, circle_y, angle, 1.0, 1.0)
end

function on_end()
    print("Game ended!")
end
