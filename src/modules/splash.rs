use ratatui::{
    Frame,
    layout::{Layout, Direction, Constraint, Rect},
    widgets::Block
};
use crate::component::Component;
pub struct Splash{
    active: bool,
}

impl Splash{
    pub fn new()-> Self{
        Self {active: true}
    }
}

impl Component for Splash{
    fn name(&self)->&str{"Splash Screen"}
    fn id(&self)->u16{0}
    fn is_active(&self)->bool{self.active}
    fn set_active(&mut self, active: bool){self.active=active}
    fn render(&mut self, frame: &mut Frame, area: Rect){
        frame.render_widget(Block::bordered().title("Splash"),area);
    }
}

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
