use bevy::prelude::Color;

#[derive(Clone, Copy)]
pub struct TerminalGlyph {
    pub(crate) glyph: u16,
    pub(crate) foreground: [f32; 4],
    pub(crate) background: [f32; 4],
}

impl Default for TerminalGlyph {
    fn default() -> Self {
        let white = Color::WHITE.to_srgba();
        let black = Color::BLACK.to_srgba();
        Self {
            glyph: 32,
            foreground: [white.red, white.green, white.blue, white.alpha],
            background: [black.red, black.green, black.blue, black.alpha],
        }
    }
}
