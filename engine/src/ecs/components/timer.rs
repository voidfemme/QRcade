use mlua::{Function, IntoLua, Lua, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimerId(pub u32);

#[derive(Debug, Clone)]
pub struct Timer {
    pub interval: f32,
    pub elapsed: f32,
    pub repeat: bool,
    pub callback: Function,
}

impl IntoLua for TimerId {
    fn into_lua(self, _lua: &Lua) -> mlua::Result<Value> {
        Ok(Value::Number(self.0 as f64))
    }
}

impl mlua::FromLua for TimerId {
    fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
        match value {
            Value::Number(n) => Ok(TimerId(n as u32)),
            _ => Err(mlua::Error::runtime("Expected number for TimerId")),
        }
    }
}
