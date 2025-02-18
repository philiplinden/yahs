#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum ColorPalette {
    VibrantRed,
    BrightOrange,
    VividYellow,
    LivelyGreen,
    DirtBrown,
    BrightBlue,
    BoldPurple,
    DarkBase,
    MediumDarkBase,
    MediumBase,
    LightBase,
    LighterBase,
    LightestBase,
    White,
    VeryLightGray,
    AlmostWhite,
}

impl ColorPalette {
    pub fn color(&self) -> Color {
        match self {
            ColorPalette::VibrantRed => Color::Srgba(Srgba::hex("#FF3D00").unwrap()),
            ColorPalette::BrightOrange => Color::Srgba(Srgba::hex("#FF6F00").unwrap()),
            ColorPalette::VividYellow => Color::Srgba(Srgba::hex("#FFD600").unwrap()),
            ColorPalette::LivelyGreen => Color::Srgba(Srgba::hex("#AEEA00").unwrap()),
            ColorPalette::DirtBrown => Color::Srgba(Srgba::hex("#795548").unwrap()),
            ColorPalette::BrightBlue => Color::Srgba(Srgba::hex("#00B0FF").unwrap()),
            ColorPalette::BoldPurple => Color::Srgba(Srgba::hex("#D5006D").unwrap()),
            ColorPalette::DarkBase => Color::Srgba(Srgba::hex("#212121").unwrap()),
            ColorPalette::MediumDarkBase => Color::Srgba(Srgba::hex("#424242").unwrap()),
            ColorPalette::MediumBase => Color::Srgba(Srgba::hex("#616161").unwrap()),
            ColorPalette::LightBase => Color::Srgba(Srgba::hex("#9E9E9E").unwrap()),
            ColorPalette::LighterBase => Color::Srgba(Srgba::hex("#BDBDBD").unwrap()),
            ColorPalette::LightestBase => Color::Srgba(Srgba::hex("#E0E0E0").unwrap()),
            ColorPalette::White => Color::Srgba(Srgba::hex("#FFFFFF").unwrap()),
            ColorPalette::VeryLightGray => Color::Srgba(Srgba::hex("#F5F5F5").unwrap()),
            ColorPalette::AlmostWhite => Color::Srgba(Srgba::hex("#FAFAFA").unwrap()),
        }
    }
}
