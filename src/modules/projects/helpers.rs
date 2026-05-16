use ratatui::{
    prelude::{Span, Modifier},
    layout::{Rect, Alignment},
    style::Style,
    widgets::{Block, BorderType, Paragraph},
    text::Line,
    Frame
};
use crate::theme::PALETTE;

pub struct HeaderConfig<'a>{
    pub name: &'a str,
    pub description: &'a str,
    pub indicators: Vec<(&'a str, bool)>
}

pub fn render_header(frame: &mut Frame, area: Rect, config: HeaderConfig){
    let mut spans = Vec::new();
    for(label, active) in config.indicators{
        let color = if active {PALETTE.success} else {PALETTE.dim};
        spans.push(Span::styled(format!("{}", label), Style::default().fg(color).add_modifier(Modifier::BOLD)));
        spans.push(Span::raw(" >> "));
    }

    let status_width: usize = spans.iter().map(|s| s.content.len()).sum();
    let desc_width = config.description.len();
    let inner_width = (area.width as usize).saturating_sub(2);
    let filler_width = inner_width.saturating_sub(status_width).saturating_sub(desc_width);
    spans.push(Span::raw(" ".repeat(filler_width)));
    spans.push(Span::styled(
        config.description, Style::default().fg(PALETTE.dim).add_modifier(Modifier::ITALIC | Modifier::DIM),
    ));
    let block = Block::bordered()
        .title(format!("{}", config.name.to_uppercase()))
        .title_alignment(Alignment::Right)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(PALETTE.border_active));
    frame.render_widget(Paragraph::new(Line::from(spans)).block(block).alignment(Alignment::Left), area);
}

pub fn styled_block()->Block<'static>{
   Block::default()
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(PALETTE.border_active))
}
