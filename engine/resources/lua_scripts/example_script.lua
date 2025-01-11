-- Global variables for snake management
local snake = {}  -- List of segments, each with an x,y position
local food = nil  -- Current food entity
local direction = {x = 0, y = 0}  -- Current movement direction
local grid_size = 32  -- Size of one grid square
local move_cooldown = 0.1  -- Time between moves
local time_since_move = 0

function on_start()
    -- Initialize snake with a single segment in the middle
    local segment = create_entity()
    set_transform(segment, 400, 300, 0, 1.0, 1.0)
    add_shape(segment, "square", 0, 255, 0)  -- Green snake
    table.insert(snake, {entity = segment, x = 400, y = 300})
    print("Snake initialized with segment: ", segment)
    
    -- Create initial food
    spawn_food()
end

function spawn_food()
    -- Remove old food if it exists
    if food then
        destroy_entity(food.entity)
    end
    
    -- Create new food at random position
    local food_entity = create_entity()
    -- Random position on a grid
    local x = math.random(2, 23) * grid_size  -- Keeping away from edges
    local y = math.random(2, 17) * grid_size
    set_transform(food_entity, x, y, 0, 1.0, 1.0)
    add_shape(food_entity, "square", 255, 0, 0)  -- Red food
    
    food = {entity = food_entity, x = x, y = y}
    print("Food spawned at: ", x, y, "with entity:", food_entity)
end

function move_snake()
    print("Moving snake. Current length: ", #snake)
    -- Calculate new head position
    local head = snake[1]
    print("Head position: ", head.x, head.y)
    local new_x = head.x + direction.x * grid_size
    local new_y = head.y + direction.y * grid_size
    print("New position will be: ", new_x, new_y)
    
    -- Create new head segment
    local new_segment = create_entity()
    set_transform(new_segment, new_x, new_y, 0, 1.0, 1.0)
    add_shape(new_segment, "square", 0, 255, 0)
    print("Created new head segment: ", new_segment)
    
    -- Insert new head at start of snake table
    table.insert(snake, 1, {entity = new_segment, x = new_x, y = new_y})
    
    -- Check for food collision
    if food and is_colliding(new_segment, food.entity) then
        print("Food eaten!")
        spawn_food()  -- Eat food and spawn new one
    else
        -- Remove tail if we didn't eat
        local tail = table.remove(snake)
        print("Removing tail segment: ", tail.entity)
        destroy_entity(tail.entity)
    end
    print("Snake length after move: ", #snake)
end

function on_frame(delta_time)
    -- Handle input
    local input_received = false
    if is_key_pressed("UP") and direction.y >= 0 then
        print("Moving UP")
        direction = {x = 0, y = -1}
        input_received = true
    elseif is_key_pressed("DOWN") and direction.y <= 0 then
        print("Moving DOWN")
        direction = {x = 0, y = 1}
        input_received = true
    elseif is_key_pressed("LEFT") and direction.x >= 0 then
        print("Moving LEFT")
        direction = {x = -1, y = 0}
        input_received = true
    elseif is_key_pressed("RIGHT") and direction.x <= 0 then
        print("Moving RIGHT")
        direction = {x = 1, y = 0}
        input_received = true
    end
    
    -- Move the snake on a timer
    if direction.x ~= 0 or direction.y ~= 0 then  -- Only if we're moving
        time_since_move = time_since_move + delta_time
        if time_since_move >= move_cooldown then
            move_snake()
            time_since_move = 0
        end
    end
end

function on_end()
    print("Game ended!")
end
