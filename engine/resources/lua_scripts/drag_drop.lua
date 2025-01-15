-- Simple drag and drop demo
-- This demonstrates basic mouse interaction with a single draggable object

-- Create our draggable object
local ball = create_entity()

-- Visual setup for our ball
local BALL_COLOR = { 0, 255, 0 } -- Green color
local BALL_RADIUS = 20

-- State for tracking drag and drop
local drag_state = {
  is_dragging = false,
  offset_x = 0, -- Distance from mouse to ball center when drag starts
  offset_y = 0,
}

-- Initialize our ball in the center of the screen
function on_start()
  -- Create a circle shape for our ball
  add_shape(ball, "circle", BALL_COLOR[1], BALL_COLOR[2], BALL_COLOR[3], { radius = BALL_RADIUS })

  -- Position the ball in the middle of the screen
  set_transform(ball, 400, 300, 0, 1.0, 1.0)

  print("Click and drag the green ball!")
end

-- Check if a point is inside our ball
function is_point_in_ball(point_x, point_y)
  -- Get the ball's current position
  local ball_x, ball_y = get_transform(ball)

  -- Calculate distance squared (faster than using square root)
  local dx = point_x - ball_x
  local dy = point_y - ball_y
  local distance_squared = dx * dx + dy * dy

  -- Check if point is within ball's radius
  return distance_squared <= BALL_RADIUS * BALL_RADIUS
end

function on_frame(delta_time)
  -- Get current mouse position (provided by engine)
  local mouse_x = _G.mouse_x or 0
  local mouse_y = _G.mouse_y or 0

  -- Check if left mouse button is pressed
  if is_mouse_pressed("LEFT") then
    if not drag_state.is_dragging then
      -- If we're not already dragging, check if we should start
      if is_point_in_ball(mouse_x, mouse_y) then
        -- Start dragging - calculate offset from mouse to ball center
        local ball_x, ball_y = get_transform(ball)
        drag_state.offset_x = ball_x - mouse_x
        drag_state.offset_y = ball_y - mouse_y
        drag_state.is_dragging = true
      end
    else
      -- Continue dragging - update ball position
      local new_x = mouse_x + drag_state.offset_x
      local new_y = mouse_y + drag_state.offset_y
      set_transform(ball, new_x, new_y, 0, 1.0, 1.0)
    end
  else
    -- Mouse button released - stop dragging
    drag_state.is_dragging = false
  end
end
