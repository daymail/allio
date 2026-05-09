//NOTE: this generalizes the info from all other modules.
use ratatui::{
    Frame,
    layout::{Layout, Direction, Constraint, Rect, Alignment},
    widgets::{Block, TitlePosition, BorderType, ListState, List, ListItem},
    style::{Style, Modifier}
};
use crate::{
    traits::Component,
    modules::{Module, ModuleCategory},
    theme::PALETTE
};

#[derive(PartialEq)]
pub enum Interaction{
    Idle,
    Hovered,
    Selected
}

pub struct ModuleItem{
    pub data: Module,
    pub interaction: Interaction,
    pub is_expanded: bool
}

#[derive(Clone, Copy)]
enum VisibleItem{
    Header(ModuleCategory),
    Submodule(usize)
}

pub struct Interface{
    active: bool,
    list_state: ListState,
    items: Vec<ModuleItem>,
    visible_map: Vec<VisibleItem>
}

//FIXME: modules have submodules eg ([+]PROJECTS has a list of projects)
impl Interface{
    pub fn new()-> Self{
        let raw_modules = crate::modules::get_all_modules();
        let items: Vec<ModuleItem> = raw_modules.into_iter().map(|m|{
            ModuleItem{
                data: m,
                interaction: Interaction::Idle,
                is_expanded: false
            }
        }).collect();
        let mut instance = Self{
            active: true,
            list_state: ListState::default().with_selected(Some(0)),
            items,
            visible_map: Vec::new()
        };
        instance.map_rebuild();
        instance.update_interactions(0);
        instance
    }
    pub fn next(&mut self){
        let i = match self.list_state.selected(){
            Some(i) => if i >= self.visible_map.len() -1 {0} else {i+1},
            None => 0
        };
        self.list_state.select(Some(i));
        self.update_interactions(i);
    }
    pub fn prev(&mut self){
        let i = match self.list_state.selected(){
            Some(i) => if i == 0 {self.visible_map.len() -1 } else {i-1},
            None => 0
        };
        self.list_state.select(Some(i));
        self.update_interactions(i);
    }
    fn map_rebuild(&mut self){
        let mut map = Vec::new();
        let categories = [
            ModuleCategory::Projects,
            ModuleCategory::Tools,
            ModuleCategory::System,
            ModuleCategory::Games,
        ];
        for cat in categories{
            let sub_indices: Vec<usize> = self.items.iter().enumerate().filter(|(_, i)| i.data.kind() == cat).map(|(idx, _)| idx).collect();
            if sub_indices.is_empty() {continue;}
            map.push(VisibleItem::Header(cat));
            let is_expanded = sub_indices.iter().any(|&i| self.items[i].is_expanded);
            if is_expanded{
                for idx in sub_indices{
                    map.push(VisibleItem::Submodule(idx));
                }
            }
        }
        self.visible_map = map;
    }
    fn update_interactions(&mut self, idx: usize){
        for item in &mut self.items{item.interaction = Interaction::Idle;}
        if let Some(VisibleItem::Submodule(data_idx)) = self.visible_map.get(idx){
            if let Some(item) = self.items.get_mut(*data_idx){
                item.interaction = Interaction::Hovered;
            }
        }
    }
    pub fn handle_enter(&mut self){
        if let Some(idx) = self.list_state.selected(){
            match self.visible_map[idx]{
                VisibleItem::Header(cat)=>{
                    let now_expanded = self.items.iter().filter(|i| i.data.kind() == cat).any(|i| i.is_expanded);
                    for item in self.items.iter_mut().filter(|i| i.data.kind() == cat){
                        item.is_expanded = !now_expanded;
                    }
                }
                VisibleItem::Submodule(idx)=>{
                    if let Some(item) = self.items.get_mut(idx){
                        item.interaction = Interaction::Selected;
                    }
                }
            }
            self.map_rebuild();
            if let Some(new_idx) = self.list_state.selected(){
                self.update_interactions(new_idx);
            }
        }
    }
    pub fn handle_hover(&mut self, idx: usize){
        if idx < self.visible_map.len(){
            self.list_state.select(Some(idx));
            self.update_interactions(idx);
        }
    }
}

impl Component for Interface{
    fn name(&self)->&str{"Interface Screen"}
    fn id(&self)->u16{0}
    fn is_active(&self)->bool{self.active}
    fn set_active(&mut self, active: bool){self.active=active}
    fn render(&mut self, frame: &mut Frame, area: Rect){
        let [main_area, status_bar] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(1)
            ]).areas(area);
        status_bar::render(frame, status_bar, &self.items);

        let [minor, major] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Fill(1)
            ]).areas(main_area);
        minor::render(frame, minor, &mut self.list_state, &self.items, &self.visible_map);
        if let Some(visual_idx) = self.list_state.selected(){
            if let Some(VisibleItem::Submodule(data_idx)) = self.visible_map.get(visual_idx){
                if let Some(item) = self.items.get(*data_idx){
                    major::render(frame, major, &item.data);
                }
            }else{
                frame.render_widget(Block::bordered().title("THIS IS THE DEFAULT MODULE BORDER").title_alignment(Alignment::Center), major);// FIXME: here, render generalized module info, ASCII art and other animations.
            }
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
    pub fn render(frame: &mut Frame, area: Rect, state: &mut ListState, items: &[ModuleItem], visible_map: &[VisibleItem]){
        let [module_area, tree] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Fill(1)
            ])
            .areas(area);

        let mut list_items = Vec::new();
        for (idx, item) in visible_map.iter().enumerate(){
            let is_selected = state.selected() == Some(idx);
            match item{
                VisibleItem::Header(cat)=>{
                    let is_expanded = items.iter().filter(|i| i.data.kind() == *cat).any(|i| i.is_expanded);
                    let icon = if is_expanded { "▼ " } else { "▶ " };
                    let style = if is_selected {
                        Style::new().fg(PALETTE.secondary).add_modifier(Modifier::BOLD)
                    } else {
                        Style::new().fg(PALETTE.dim)
                    };
                    list_items.push(ListItem::new(format!("{}{}", icon, cat.as_str())).style(style));
                }
                VisibleItem::Submodule(sub_idx) =>{
                    let item = &items[*sub_idx];
                    let sub_indices: Vec<usize> = items.iter().enumerate().filter(|(_, i)| i.data.kind() == item.data.kind()).map(|(idx, _)| idx).collect();
                    let is_last = sub_indices.last() == Some(sub_idx);
                    let prefix = if is_last { "  └─ " } else { "  ├─ " };
                    let style = if is_selected {
                        Style::new().fg(PALETTE.text).bg(PALETTE.selection)
                    } else {
                        Style::new().fg(PALETTE.text)
                    };
                    list_items.push(ListItem::new(format!("{}{}", prefix, item.data.submodname())).style(style));
                }
            }
        }
        let list_widget = List::new(list_items)
            .block(Block::bordered()
                .title("MODULES")
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(PALETTE.border_active))
                .style(Style::new().bg(PALETTE.surface))
                .title_alignment(Alignment::Center)
            )
            .highlight_style(Style::default().bg(PALETTE.selection));
        frame.render_stateful_widget(list_widget, module_area, state);
    }
}

mod status_bar{
    use super::*;
    pub fn render(frame: &mut Frame, area: Rect, items: &[ModuleItem]){

    }
}
