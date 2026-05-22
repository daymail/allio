use ratatui::{
    prelude::{Span, Modifier},
    layout::{Rect, Alignment, Direction, Layout, Constraint},
    style::{Style},
    widgets::{Block, BorderType, Paragraph},
    text::Line,
    Frame
};
use crate::theme::Theme;

pub struct HeaderConfig<'a>{
    pub name: &'a str,
    pub description: &'a str,
    pub indicators: Vec<(&'a str, bool)>
}

//default header config
pub fn render_header(frame: &mut Frame, area: Rect, config: HeaderConfig, theme: &Theme){
    let mut spans = Vec::new();
    for(label, active) in config.indicators{
        let color = if active {theme.success} else {theme.dim};
        spans.push(Span::styled(format!("{}", label), Style::default().fg(color).add_modifier(Modifier::BOLD)));
        spans.push(Span::raw(" >> "));
    }

    let status_width: usize = spans.iter().map(|s| s.content.len()).sum();
    let desc_width = config.description.len();
    let inner_width = (area.width as usize).saturating_sub(2);
    let filler_width = inner_width.saturating_sub(status_width).saturating_sub(desc_width);
    spans.push(Span::raw(" ".repeat(filler_width)));
    spans.push(Span::styled( config.description, Style::default().fg(theme.dim).add_modifier(Modifier::ITALIC | Modifier::DIM),));
    let title = Span::styled(format!("{}", config.name.to_uppercase()), Style::default().bg(theme.secondary).fg(theme.border_inactive));
    let block = Block::bordered()
        .title(title)
        .title_alignment(Alignment::Right)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme.border_active));
    frame.render_widget(Paragraph::new(Line::from(spans)).block(block).alignment(Alignment::Left), area);
}

pub fn centered_rect(perc_x: u16, perc_y: u16, r: Rect) -> Rect{
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - perc_y)/2),
            Constraint::Percentage(perc_y),
            Constraint::Percentage((100 - perc_y)/2)
        ]).split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - perc_x)/2),
            Constraint::Percentage(perc_x),
            Constraint::Percentage((100 - perc_x)/2)
        ]).split(popup_layout[1])[1]
}
