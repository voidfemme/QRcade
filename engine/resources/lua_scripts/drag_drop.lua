-- Create our draggable object
local ball = create_entity()

-- Visual setup for our ball
local BALL_COLOR = { 0, 255, 0 } -- Green color
local BALL_RADIUS = 20

function on_start()
	-- Create a circle shape for our ball
	add_shape(ball, "circle", BALL_COLOR[1], BALL_COLOR[2], BALL_COLOR[3], { radius = BALL_RADIUS })

	-- Position the ball in the middle of the screen
	set_transform(ball, 400, 300, 0, 1.0, 1.0)

	print("Click and drag the green ball!")
end

function on_frame(delta_time)
    local mouse_x = _G.mouse_x or 0
    local mouse_y = _G.mouse_y or 0

    if is_mouse_pressed("LEFT") then
        if not is_dragging(ball) then
            -- The dragging will now be handled by the input system
            local entity = can_drag_entity(mouse_x, mouse_y)
            if entity then
                start_drag(entity, mouse_x, mouse_y)
            end
        else
            update_drag(mouse_x, mouse_y)
        end
    else
        end_drag()
    end
end
