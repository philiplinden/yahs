use ratatui::{
    prelude::Alignment,
    widgets::{Block, Borders, Paragraph},
    text::{Line, Text},
    style::Style,
    Frame,
};
use yahs::prelude::*;
use crate::colors::TuiColor;

pub fn draw_title(frame: &mut Frame<'_>, area: ratatui::layout::Rect) {
    let title = Paragraph::new("yet another hab simulator 🎈")
        .style(Style::default().fg(TuiColor::Title.color()))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, area);
}

pub fn draw_status(
    frame: &mut Frame<'_>, 
    area: ratatui::layout::Rect,
    state: &SimState,
    time: f32,
    time_options: &TimeScaleOptions,
) {
    let status = format!(
        "state: {:?} | time: {:.2}s | speed: {}x",
        state,
        time,
        time_options.multiplier,
    );
    let status = Paragraph::new(status)
        .style(Style::default().fg(TuiColor::Status.color()))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(status, area);
}

pub fn draw_balloon_data(
    frame: &mut Frame<'_>,
    area: ratatui::layout::Rect,
    balloons: &[(
        &bevy::prelude::Name,
        &bevy::prelude::Transform,
        &Weight,
        &Buoyancy,
        &Drag,
        &IdealGas,
    )],
) {
    let mut balloon_data = Vec::new();
    for (name, transform, weight, buoyancy, drag, gas) in balloons {
        balloon_data.extend_from_slice(&[
            format!("{}", name.as_str()),
            format!("position: {:?} m", transform.translation),
            format!("gas density: {:.2} kg/m³", gas.density.kg_per_m3()),
            format!("volume: {:.2} m³", gas.volume().m3()),
            Line::from(format!("weight: {:.2} N", weight.force()))
                .style(Style::default().fg(TuiColor::Weight.color())).to_string(),
            Line::from(format!("buoyancy: {:.2} N", buoyancy.force()))
                .style(Style::default().fg(TuiColor::Buoyancy.color())).to_string(),
            Line::from(format!("drag: {:.2} N", drag.force()))
                .style(Style::default().fg(TuiColor::Drag.color())).to_string(),
        ]);
    }
    
    let balloon_text = Paragraph::new(Text::from(balloon_data.join("\n")))
        .block(Block::default().borders(Borders::ALL).title("balloon status"));
    frame.render_widget(balloon_text, area);
}

pub fn draw_controls(frame: &mut Frame<'_>, area: ratatui::layout::Rect) {
    let controls = Paragraph::new(
        "q: quit | space: pause/play | r: reset | ←/→: speed"
    )
        .style(Style::default().fg(TuiColor::Controls.color()))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(controls, area);
} 
