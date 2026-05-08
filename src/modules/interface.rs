//NOTE: this generalizes the info from all other modules.
use ratatui::{
    Frame,
    layout::{Layout, Direction, Constraint, Rect, Alignment},
    widgets::{Block, TitlePosition, BorderType, ListState, List, ListItem},
    style::{Style, Modifier}
};
use crate::traits::{Component, ProjectModule};
use crate::theme::PALETTE;

pub enum Module{
    Project(Box<dyn ProjectModule>),
    //TODO: add other modules here
}

impl Module{
    fn name(&self)->&str{
        match self{
            Module::Project(p) => p.category()
        }
    }
}

pub struct Interface{
    active: bool,
    list_state: ListState,
    modules: Vec<Module>
}

//FIXME: modules have submodules eg ([+]PROJECTS has a list of projects)
impl Interface{
    pub fn new()-> Self{
        Self{
            active: true,
            list_state: ListState::default().with_selected(Some(0)),
            modules: crate::modules::get_all_modules()
        }
    }
    pub fn next(&mut self){
        let i = match self.list_state.selected(){
            Some(i) => if i >= self.modules.len() -1 {0} else {i+1},
            None => 0
        };
        self.list_state.select(Some(i));
    }
    pub fn prev(&mut self){
        let i = match self.list_state.selected(){
            Some(i) => if i == 0 {self.modules.len() -1 } else {i-1},
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
        minor::render(frame, minor, &mut self.list_state, &self.modules);
        if let Some(i) = self.list_state.selected(){
            if let Some(module) = self.modules.get(i){
                major::render(frame, major, module);
            }
        }else{
            frame.render_widget(Block::bordered().title("NO MODULE SELECTED"), major);
        }
    }
}

mod major{
    use super::*;
    pub fn render(frame: &mut Frame, area: Rect, module: &Module){
        frame.render_widget(Block::bordered(), area)
    }
}

mod minor{
    use super::*;
    pub fn render(frame: &mut Frame, area: Rect, state: &mut ListState, module: &Vec<Module>){
        let [module_area, tree] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Fill(1)
            ])
            .areas(area);
        draw_module(frame, module_area, state, module);
    }

    fn draw_module(frame: &mut Frame, area: Rect, state: &mut ListState, modules: &[Module]){
        let mut items = Vec::new();
        if let Some(first) = modules.first(){
            items.push(
                    ListItem::new(format!("▼ {}", first.name()))
                    .style(Style::new().fg(PALETTE.secondary).add_modifier(Modifier::BOLD))
            );
        }
        for(i, m) in modules.iter().enumerate(){
            let is_last = i == modules.len() - 1;
            let prefix = if is_last { "  └─ " } else { "  ├─ " };
            let sub_name = match m{
                Module::Project(p) => p.submodname(),
            };
            items.push(
                ListItem::new(format!("{}{}", prefix, sub_name))
                    .style(Style::new().fg(PALETTE.text))
            );
        }
        let list_widget = List::new(items)
            .block(Block::bordered()
                .title("Modules")
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(PALETTE.border_active))
                .style(Style::new().bg(PALETTE.surface))
                .title_position(TitlePosition::Top)
                .title_alignment(Alignment::Center),
            )
            .highlight_style(Style::default().fg(PALETTE.highlight).add_modifier(Modifier::BOLD));
        frame.render_stateful_widget(list_widget, area, state);
    }
}

