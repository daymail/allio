//NOTE: this generalizes the info from all other modules.
use ratatui::{
    Frame,
    layout::{Layout, Direction, Constraint, Rect, Alignment},
    widgets::{Block, TitlePosition, BorderType, ListState, List, ListItem},
    style::{Color, Style, Modifier}
};
use crate::component::Component;

pub struct Interface{
    active: bool,
    list_state: ListState,
    items: Vec<String>
}

//FIXME: modules have submodules eg ([+]PROJECTS has a list of projects)
impl Interface{
    pub fn new()-> Self{
        let mut state = ListState::default();
        state.select(Some(0));
        Self{
            active: true,
            list_state: state,
            items: vec![
                "PROJECTS".to_string(),
                "DOCKER".to_string(),
                "MUSIC".to_string(),
                "TOP".to_string(),
                "GAMES".to_string()
                //NOTE: each of these has submodules brought to view by clicking.
            ]
        }
    }
    pub fn next(&mut self){
        let i = match self.list_state.selected(){
            Some(i) => if i >= self.items.len() -1 {0} else {i+1},
            None => 0
        };
        self.list_state.select(Some(i));
    }
    pub fn prev(&mut self){
        let i = match self.list_state.selected(){
            Some(i) => if i == 0 {self.items.len() -1 } else {i-1},
            None => 0
        };
        self.list_state.select(Some(i));
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
                Constraint::Percentage(20),
                Constraint::Fill(1)
            ]).areas(area);
        minor::render(frame, minor, &mut self.list_state, &self.items);
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
    pub fn render(frame: &mut Frame, area: Rect, state: &mut ListState, items: &[String]){
        let [module, tree] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Fill(1)
            ])
            .areas(area);
        draw_module(frame, module, state, items);
    }

    fn draw_module(frame: &mut Frame, area: Rect, state: &mut ListState, items: &[String]){
        let list_items: Vec<ListItem> = items
            .iter()
            .map(|i| ListItem::new(i.as_str()))
            .collect();
        let list_widget = List::new(list_items)
            .block(Block::bordered()
                .title("Modules")
                .border_type(BorderType::Rounded)
                .title_position(TitlePosition::Top)
                .title_alignment(Alignment::Center),
            )
            .highlight_symbol("*")
            .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
        frame.render_stateful_widget(list_widget, area, state);
    }
}





