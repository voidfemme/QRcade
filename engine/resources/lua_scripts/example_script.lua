-- Create an entity with a red rectangle
function on_start()
    -- Changed from entity.create() to create_entity()
    local e = create_entity()  
    set_transform(e, 200, 300, 0, 1.0, 1.0)
    add_rectangle(e, 50, 50, 255, 0, 0)
end

-- Move the entity every frame
function on_frame(delta_time)
    local e = 1 -- Assuming the entity ID is 1
    local x, y, rotation = get_transform(e)
    set_transform(e, x + 100 * delta_time, y, rotation, 1.0, 1.0)
end

-- End game logic
function on_end()
    print("Game ended!")
end
