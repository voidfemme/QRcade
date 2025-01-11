local player, goal
local player_x, player_y
local goal_x, goal_y
local speed = 200  -- Movement speed in pixels per second
local screen_width = 800
local screen_height = 600

local followers = {}        -- List to hold follower segments
local follow_distance = 16  -- Distance each follower should maintain

function spawn_goal()
    -- Create a new red square at a random position
    goal = create_entity()
    goal_x = math.random(50, screen_width - 50)   -- Keeping a margin
    goal_y = math.random(50, screen_height - 50)  -- Keeping a margin
    set_transform(goal, goal_x, goal_y, 0, 1.0, 1.0)
    add_shape(goal, "square", 255, 0, 0)  -- Red square
end

function on_start()
    -- Create player as a green circle
    player = create_entity()
    player_x = 100
    player_y = 100
    set_transform(player, player_x, player_y, 0, 1.0, 1.0)
    add_shape(player, "circle", 0, 255, 0)  -- Green circle

    spawn_goal()  -- Create the initial goal
end

function on_frame(delta_time)
    local moveX, moveY = 0, 0

    -- Check for arrow key input to determine movement direction
    if is_key_pressed("LEFT") then
        moveX = moveX - 1
    end
    if is_key_pressed("RIGHT") then
        moveX = moveX + 1
    end
    if is_key_pressed("UP") then
        moveY = moveY - 1
    end
    if is_key_pressed("DOWN") then
        moveY = moveY + 1
    end

    -- Normalize the movement vector to maintain consistent speed diagonally
    local magnitude = math.sqrt(moveX * moveX + moveY * moveY)
    if magnitude > 0 then
        moveX = moveX / magnitude
        moveY = moveY / magnitude
    end

    -- Update player position based on input and speed
    player_x = player_x + moveX * speed * delta_time
    player_y = player_y + moveY * speed * delta_time
    set_transform(player, player_x, player_y, 0, 1.0, 1.0)

    -- Check for collision between the player and the goal
    if goal and is_colliding(player, goal) then
        destroy_entity(goal)  -- Remove the old goal
        print("Goal reached, preparing new elements")

        -- Create a new follower before spawning a new goal
        local new_follower_entity = create_entity()
        set_transform(new_follower_entity, player_x, player_y, 0, 1.0, 1.0)
        add_shape(new_follower_entity, "circle", 0, 200, 0)  -- Dark green circle
        table.insert(followers, {entity = new_follower_entity, x = player_x, y = player_y})

        spawn_goal()  -- Spawn a new goal at a random position
        print("New goal spawned")
    end

    -- Update followers to follow the previous circle 16 pixels away
    for i, follower in ipairs(followers) do
        local leader_x, leader_y
        if i == 1 then
            leader_x, leader_y = player_x, player_y
        else
            leader_x, leader_y = followers[i-1].x, followers[i-1].y
        end

        local dx = leader_x - follower.x
        local dy = leader_y - follower.y
        local distance = math.sqrt(dx*dx + dy*dy)

        if distance > follow_distance then
            local dir_x = dx / distance
            local dir_y = dy / distance
            follower.x = leader_x - dir_x * follow_distance
            follower.y = leader_y - dir_y * follow_distance
            set_transform(follower.entity, follower.x, follower.y, 0, 1.0, 1.0)
        end
    end
end

function on_end()
    print("Game ended")
end

