
function on_start()
    -- Create a red square with custom width and height
    local square_entity = create_entity()
    set_transform(square_entity, 100, 100, 0, 1.0, 1.0)
    add_shape(square_entity, "square", 255, 0, 0, { width = 64, height = 64 })

    -- Create a green circle with a custom radius
    local circle_entity = create_entity()
    set_transform(circle_entity, 300, 100, 0, 1.0, 1.0)
    add_shape(circle_entity, "circle", 0, 255, 0, { radius = 32 })

    -- Create a blue triangle with custom vertices
    local triangle_entity = create_entity()
    set_transform(triangle_entity, 500, 100, 0, 1.0, 1.0)
    add_shape(triangle_entity, "triangle", 0, 0, 255, {
        x1 = 0,   y1 = -20,
        x2 = -20, y2 = 20,
        x3 = 20,  y3 = 20
    })

    -- Create a yellow line with a custom end point
    local line_entity = create_entity()
    set_transform(line_entity, 700, 100, 0, 1.0, 1.0)
    add_shape(line_entity, "line", 255, 255, 0, { x2 = 50, y2 = 0 })
end

function on_frame(delta_time)
    -- Update logic or animations can go here if needed.
end

function on_end()
    print("Script ended.")
end
