fn centered_rect(area: Rect, width: u16, height: u16)->Rect{
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(height),
            Constraint::Fill(1)
        ])
        .split(area);
    let center = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(width),
            Constraint::Fill(1)
        ])
        .split(vertical[1]);
    center[1]
}
