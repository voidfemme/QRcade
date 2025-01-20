#[derive(Debug, Clone, Copy)]
pub enum TextId {
    GameOver,
    PressSpace,
    Score,
    YouWin,
    GamePaused,
    PressEscToQuit,
    Lives,
    Level,
    Ready,
    Go,
    HighScore,
    NewHighScore,
    TryAgain,
}

impl TextId {
    pub fn get_text(&self) -> &'static str {
        match self {
            TextId::GameOver => "Game Over",
            TextId::PressSpace => "Press Space",
            TextId::Score => "Score: ",
            TextId::YouWin => "You win!",
            TextId::GamePaused => "Game Paused",
            TextId::PressEscToQuit => "Press ESC to Quit",
            TextId::Lives => "Lives: ",
            TextId::Level => "Level: ",
            TextId::Ready => "Ready",
            TextId::Go => "Go!",
            TextId::HighScore => "High Score: ",
            TextId::NewHighScore => "New High Score!",
            TextId::TryAgain => "Press Space to Try Again",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TextAlignment {
    pub h_align: HorizontalAlign,
    pub v_align: VerticalAlign,
}

#[derive(Debug, Clone, Copy)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

impl Default for TextAlignment {
    fn default() -> Self {
        Self {
            h_align: HorizontalAlign::Left,
            v_align: VerticalAlign::Top,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Text {
    pub text_id: TextId,
    pub color: (u8, u8, u8),
    pub scale: f32,
    pub alignment: TextAlignment,
    pub visible: bool,
    pub value: Option<String>,
}

impl Text {
    pub fn new(text_id: TextId) -> Self {
        Self {
            text_id,
            color: (255, 255, 255),
            scale: 1.0,
            alignment: TextAlignment::default(),
            visible: true,
            value: None,
        }
    }

    pub fn with_value(mut self, value: impl ToString) -> Self {
        self.value = Some(value.to_string());
        self
    }

    pub fn get_string(&self) -> String {
        let base = self.text_id.get_text();
        if let Some(value) = &self.value {
            format!("{}{}", base, value)
        } else {
            base.to_string()
        }
    }

    pub fn with_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.color = (r, g, b);
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_alignment(mut self, h_align: HorizontalAlign, v_align: VerticalAlign) -> Self {
        self.alignment = TextAlignment { h_align, v_align };
        self
    }

    pub fn set_visibility(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_text_value(&mut self, value: String) {
        self.value = Some(value);
    }
}
