-------------------------------------------------------
-- Global variables
-------------------------------------------------------
player = nil
asteroids = {}
bullets = {}

-- We'll store the ship's facing angle separately so that
-- if the ship slows down or stops, it doesn't "snap" to 0°.
local shipAngle = 0

-------------------------------------------------------
-- Utility: spawn a single asteroid
-------------------------------------------------------
local function spawn_asteroid()
  local asteroid = create_entity()
  local x = math.random(0, 800)  -- Example: 800 wide window
  local y = math.random(0, 600)  -- Example: 600 tall window
  set_transform(asteroid, x, y, 0, 1, 1)
  add_shape(asteroid, "circle", 255, 255, 255, {radius = 20})

  local vx = math.random(-50, 50)
  local vy = math.random(-50, 50)
  set_velocity(asteroid, vx, vy)

  table.insert(asteroids, asteroid)
end

-------------------------------------------------------
-- on_start: Called once at game start
-------------------------------------------------------
function on_start()
  math.randomseed(os.time())

  -- Create the player's spaceship
  player = create_entity()
  set_transform(player, 400, 300, 0, 1, 1) -- roughly center screen
  add_shape(player, "triangle", 0, 255, 0,
    {
      x1 = 0,   y1 = -15,
      x2 = -10, y2 = 10,
      x3 = 10,  y3 = 10
    })
  set_velocity(player, 0, 0)

  -- Spawn a few asteroids
  for i = 1, 5 do
    spawn_asteroid()
  end
end

-------------------------------------------------------
-- on_frame: Called every frame
-------------------------------------------------------
function on_frame(delta_time)
  -- Get player's current transform and velocity
  local x, y, rotation, sx, sy = get_transform(player)
  local vx, vy = get_velocity(player)

  -- Safeguard against nil scale
  sx = sx or 1
  sy = sy or 1

  -- Movement parameters
  local speed    = 200  -- base acceleration in each WASD direction
  local friction = 0.98 -- factor to dampen velocity each frame

  -- WASD Movement (absolute, screen-based)
  if is_key_pressed("W") then
    vy = vy - (speed * delta_time)
  end
  if is_key_pressed("S") then
    vy = vy + (speed * delta_time)
  end
  if is_key_pressed("A") then
    vx = vx - (speed * delta_time)
  end
  if is_key_pressed("D") then
    vx = vx + (speed * delta_time)
  end

  -- Apply mild friction
  vx = vx * friction
  vy = vy * friction

  -- Update the facing angle based on velocity (only if velocity is non-zero)
  -- In screen coords: vy negative is "up," vy positive is "down," etc.
  -- atan2(vy, vx) returns angle in radians relative to positive X-axis.
  -- We add 90 so that 0° is "up."
  if math.abs(vx) > 0.01 or math.abs(vy) > 0.01 then
    shipAngle = math.deg(math.atan2(vy, vx)) + 90
  end

  -- Update the player's velocity and transform
  set_velocity(player, vx, vy)
  set_transform(player, x, y, shipAngle, sx, sy)

  -- Space to shoot bullets
  if is_key_pressed("SPACE") then
    local bullet = create_entity()
    set_transform(bullet, x, y, 0, 1, 1)
    add_shape(bullet, "circle", 255, 0, 0, {radius = 2})

    -- Fire in the direction the ship is facing
    local bulletSpeed = 300
    -- Convert degrees back to radians. Because we set
    -- 0° = "up," we subtract 90° so forward is aligned with negative Y.
    local rad = math.rad(shipAngle - 90)
    local bulletVx = math.cos(rad) * bulletSpeed
    local bulletVy = math.sin(rad) * bulletSpeed

    set_velocity(bullet, bulletVx, bulletVy)
    table.insert(bullets, bullet)
  end

  -- Check for bullet-asteroid collisions
  for bi = #bullets, 1, -1 do
    local bullet = bullets[bi]
    for ai = #asteroids, 1, -1 do
      local asteroid = asteroids[ai]
      if is_colliding(bullet, asteroid) then
        destroy_entity(bullet)
        destroy_entity(asteroid)
        table.remove(bullets, bi)
        table.remove(asteroids, ai)
        break
      end
    end
  end

  -- Check for player-asteroid collisions
  for _, asteroid in ipairs(asteroids) do
    if is_colliding(player, asteroid) then
      print("Player hit by asteroid!")
      -- Insert damage, lives, or game-over logic here
    end
  end

  -- (Optional) handle wrapping, off-screen bullets, etc.
end

-------------------------------------------------------
-- on_end: Called when the game ends or script unloads
-------------------------------------------------------
function on_end()
  print("Game over or cleaning up resources.")
end

