use ratatui::style::Color;

#[derive(Debug, Clone, Copy)]
pub enum TuiColor {
    // UI Elements
    Title,
    Status,
    Controls,
    
    // Forces (matching yahs-ui colors)
    Weight,
    Buoyancy,
    Drag,
}

impl TuiColor {
    pub fn color(&self) -> Color {
        match self {
            // UI Elements
            TuiColor::Title => Color::Cyan,
            TuiColor::Status => Color::Yellow,
            TuiColor::Controls => Color::Green,
            
            // Forces - matching yahs-ui/src/forces/body.rs colors
            TuiColor::Weight => Color::Red,
            TuiColor::Buoyancy => Color::Blue,
            TuiColor::Drag => Color::Green,
        }
    }
} 
