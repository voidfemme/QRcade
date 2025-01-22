# [Timer](Timer) API

[Back to index](index.md)

The timer API allows you to schedule and manage timed events in your game. Use this API to create
repeating or one-shot timers for game mechanics, animations, or any time-based
functionality.

## Functions:

### `timer.set_interval(callback, interval, repeat)`
Creates a new timer that executes a callback function at specified intervals.

**Parameters**:
- `callback` (function): Function to execute when the timer triggers
- `interval` (number): Time in seconds between executions
- `repeat` (bool): Whether the timer should repeat or execute only once

**Returns**:
- `timer_id` (number): Unique identifier for the created timer

**Example**:
```lua
-- Create a repeating timer that prints every second
local timer_id = timer.set_interval(function()
  print("Timer tick!")
end, 1.0, true)

-- Create a one-shot timer that triggers after 5 seconds
local one_shot = timer.set_interval(function()
  print("5 seconds elapsed!")
end, 5.0, false)
)
```

### `timer.clear(timer_id)`

Stops and removes a timer.

**Parameters**:
- `timer_id` (number): The ID of the timer to clear

**Example**:
```lua
-- Create a timer and store its ID 
local timer_id = timer.set_interval(function()
  print("Tick")
end, 1.0, true)

-- Later, stop the timer
timer.clear(timer_id)
```

## Best Practices

### Managing Multiple Timers
Track timer IDs when creating multiple timers:
```lua
local timers = {}

-- Create timers with different intervals
timers.spawn = timer.set_interval(function()
  spawn_enemy()
end, 5.0, true)

timers.score = timer.set_interval(function()
  update_score()
end, 1.0, true)

-- Clear all timers when done
for _, id in pairs(timers) do 
  timer.clear(id)
end
```

### Error Handling
Timer callbacks should include error handling:
```lua
timer.set_interval(function()
  local success, error = pcall(function()
    -- Your timer logic here
  end
end, 1.0, true)
)
```
