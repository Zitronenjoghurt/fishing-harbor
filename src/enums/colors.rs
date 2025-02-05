use poise::serenity_prelude::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Colors {
    Orange,
    Success,
}

impl Colors {
    pub fn as_color(&self) -> Color {
        match self {
            Colors::Orange => Color::from_rgb(255, 195, 132),
            Colors::Success => Color::from_rgb(176, 235, 147),
        }
    }
}

impl From<Colors> for Color {
    fn from(value: Colors) -> Self {
        value.as_color()
    }
}
