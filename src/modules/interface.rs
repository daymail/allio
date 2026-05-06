//NOTE: this generalizes the info from all other modules.
use ratatui::{
    Frame,
    layout::{Layout, Direction, Constraint, Rect, Alignment},
    widgets::{Block, TitlePosition, BorderType},
    style::{Color, Style, Modifier}
};
use crate::component::Component;

pub struct Interface{
    active: bool,
}

impl Interface{
    pub fn new()-> Self{
        Self {active: true}
    }
}

impl Component for Interface{
    fn name(&self)->&str{"Interface Screen"}
    fn id(&self)->u16{0}
    fn is_active(&self)->bool{self.active}
    fn set_active(&mut self, active: bool){self.active=active}
    fn render(&mut self, frame: &mut Frame, area: Rect){
        let [minor, major] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70)
            ]).areas(area);
        minor::render(frame, minor);
        major::render(frame, major);
    }
}

mod major{
    use super::*;
    pub fn render(frame: &mut Frame, area: Rect){
        frame.render_widget(Block::bordered(), area)
    }
}

mod minor{
    use super::*;
    pub fn render(frame: &mut Frame, area: Rect){
        let [module, tree] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Fill(1)
            ])
            .areas(area);
        draw_module(frame, module);
    }

    fn draw_module(frame: &mut Frame, area: Rect){
        frame.render_widget(Block::bordered()
            .title("Modules")
            .border_type(BorderType::Rounded)
            .title_position(TitlePosition::Top)
            .title_alignment(Alignment::Center),
            area
        );
    }
}
