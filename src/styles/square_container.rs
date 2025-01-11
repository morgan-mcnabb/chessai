
use iced::widget::container::{Appearance, StyleSheet as ContainerStyleSheet};
use iced::{Color, Theme};

#[derive(Debug, Clone, Copy)]
pub struct SquareContainer {
    pub color: Color,
}

impl SquareContainer {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl ContainerStyleSheet for SquareContainer {
    type Style = Theme;

    fn appearance(&self, _theme: &Self::Style) -> Appearance {
        Appearance {
            background: Some(self.color.into()),
            ..Default::default()
        }
    }
}

