#[derive(Clone, Debug)]
pub struct Draggable {
    pub enabled: bool,
}

impl Draggable {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    pub fn disabled() -> Self {
        Self { enabled: false }
    }

    pub fn set_enabled(&mut self, enabled: bool) -> Self {
        Self { enabled }
    }
}
