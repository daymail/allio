//NOTE: this generalizes the info from all other modules.
use crate::{
    modules::{Module, ModuleCategory},
    theme::Theme,
    traits::{BaseModule, Component},
};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseEvent, MouseEventKind, MouseButton};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, List, ListItem, ListState, TitlePosition},
};

#[derive(PartialEq)]
pub enum Interaction {
    Idle,
    Hovered,
    Selected,
}

#[derive(PartialEq, Clone, Copy)]
pub enum InterfaceMode {
    Normal,
    Insert,
}

pub struct ModuleItem {
    pub data: Module,
    pub interaction: Interaction,
    pub is_expanded: bool,
}

#[derive(Clone, Copy)]
enum VisibleItem {
    Header(ModuleCategory),
    Submodule(usize),
}

pub struct Interface {
    active: bool,
    list_state: ListState,
    items: Vec<ModuleItem>,
    visible_map: Vec<VisibleItem>,
    pub mode: InterfaceMode,
    pub theme: Theme,
    minor_area: Rect,
    major_area: Rect,
    pub fullscreen: bool
}

//FIXME: modules have submodules eg ([+]PROJECTS has a list of projects)
impl Interface {
    pub fn new() -> Self {
        let raw_modules = crate::modules::get_all_modules();
        let items: Vec<ModuleItem> = raw_modules
            .into_iter()
            .map(|m| ModuleItem {
                data: m,
                interaction: Interaction::Idle,
                is_expanded: false,
            })
            .collect();
        let mut instance = Self {
            active: true,
            list_state: ListState::default().with_selected(Some(0)),
            items,
            visible_map: Vec::new(),
            mode: InterfaceMode::Normal,
            theme: Theme::new(),
            minor_area: Rect::default(),
            major_area: Rect::default(),
            fullscreen: false
        };
        instance.theme.theme_refresh();
        instance.map_rebuild();
        instance.update_interactions(0);
        instance
    }
    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.visible_map.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.update_interactions(i);
    }
    pub fn prev(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.visible_map.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.update_interactions(i);
    }
    fn map_rebuild(&mut self) {
        let mut map = Vec::new();
        let categories = [
            ModuleCategory::Projects,
            ModuleCategory::Tools,
            ModuleCategory::Games
        ];
        for cat in categories {
            let sub_indices: Vec<usize> = self
                .items
                .iter()
                .enumerate()
                .filter(|(_, i)| i.data.kind() == cat)
                .map(|(idx, _)| idx)
                .collect();
            if sub_indices.is_empty() {
                continue;
            }
            map.push(VisibleItem::Header(cat));
            let is_expanded = sub_indices.iter().any(|&i| self.items[i].is_expanded);
            if is_expanded {
                for idx in sub_indices {
                    map.push(VisibleItem::Submodule(idx));
                }
            }
        }
        self.visible_map = map;
    }
    fn update_interactions(&mut self, idx: usize) {
        for item in &mut self.items {
            item.interaction = Interaction::Idle;
        }
        if let Some(VisibleItem::Submodule(data_idx)) = self.visible_map.get(idx) {
            if let Some(item) = self.items.get_mut(*data_idx) {
                item.interaction = Interaction::Hovered;
            }
        }
    }
    pub fn handle_enter(&mut self) {
        if let Some(idx) = self.list_state.selected() {
            match self.visible_map[idx] {
                VisibleItem::Header(cat) => {
                    let now_expanded = self
                        .items
                        .iter()
                        .filter(|i| i.data.kind() == cat)
                        .any(|i| i.is_expanded);
                    for item in self.items.iter_mut().filter(|i| i.data.kind() == cat) {
                        item.is_expanded = !now_expanded;
                    }
                }
                VisibleItem::Submodule(idx) => {
                    if let Some(item) = self.items.get_mut(idx) {
                        item.interaction = Interaction::Selected;
                    }
                }
            }
            self.map_rebuild();
            if let Some(new_idx) = self.list_state.selected() {
                self.update_interactions(new_idx);
            }
        }
    }
    pub fn handle_hover(&mut self, idx: usize) {
        if idx < self.visible_map.len() {
            self.list_state.select(Some(idx));
            self.update_interactions(idx);
        }
    }

    pub fn mouse_handler(&mut self, mouse: MouseEvent){
        let x = mouse.column;
        let y = mouse.row;

        if self.minor_area.contains(ratatui::layout::Position::new(x, y)){
            let rel_y = y.saturating_sub(self.minor_area.y + 1) as usize;
            match mouse.kind{
                MouseEventKind::Down(MouseButton::Left) =>{
                    if rel_y < self.visible_map.len(){
                        self.list_state.select(Some(rel_y));
                        self.update_interactions(rel_y);
                        self.handle_enter();
                    }
                }
                MouseEventKind::ScrollUp => self.prev(),
                MouseEventKind::ScrollDown => self.next(),
                _ => {}
            }
        }else if self.major_area.contains(ratatui::layout::Position::new(x, y)){
            if let Some(visual_idx) = self.list_state.selected(){
                if let Some(VisibleItem::Submodule(data_idx)) = self.visible_map.get(visual_idx){
                    if let Some(item) = self.items.get_mut(*data_idx){
                        item.data.handle_mouse(mouse, self.mode);
                    }
                }
            }
        }
    }
}

impl Component for Interface {
    fn name(&self) -> &str {
        "Interface Screen"
    }
    fn id(&self) -> u16 {
        0
    }
    fn is_active(&self) -> bool {
        self.active
    }
    fn set_active(&mut self, active: bool) {
        self.active = active
    }
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let root = Block::default().style(Style::default().bg(self.theme.surface));
        frame.render_widget(&root, area);
        let workspace = root.inner(area);
        if self.fullscreen{
            self.major_area = workspace;
            self.minor_area = Rect::default();
            if let Some(visual_idx) = self.list_state.selected(){
                if let Some(VisibleItem::Submodule(data_idx)) = self.visible_map.get(visual_idx){
                    if let Some(item) = self.items.get_mut(*data_idx){
                        major::render(frame, self.major_area, Rect::default(), &mut item.data, &self.theme);
                    }
                }
            }
        }else{
            let [minor, major] = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Fill(1)])
                .areas(workspace);

            let [list_area, sidepanel] = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(30), Constraint::Fill(1)])
                .areas(minor);

            self.minor_area = list_area;
            self.major_area = major;

            minor::render( frame, list_area, &mut self.list_state, &self.items, &self.visible_map, &self.mode, &self.theme);
            if let Some(visual_idx) = self.list_state.selected() {
                if let Some(VisibleItem::Submodule(data_idx)) = self.visible_map.get(visual_idx) {
                    if let Some(item) = self.items.get_mut(*data_idx) {
                        major::render(frame, major, sidepanel, &mut item.data, &self.theme);
                    }
                } else {
                    frame.render_widget(
                        Block::bordered()
                            .title("THIS IS THE DEFAULT MODULE BORDER")
                            .title_alignment(Alignment::Center),
                        major,
                    ); // FIXME: here, render generalized module info, ASCII art and other animations.
                }
            }
            update_cursor_style(&self.mode);
        }
    }
    fn event_handler(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {return;}
        match self.mode {
            InterfaceMode::Normal => match key.code {
                KeyCode::Char(':') | KeyCode::Char('i') | KeyCode::Char('a') => self.mode = InterfaceMode::Insert,
                KeyCode::Char('/') =>{
                    if let Some(visual_idx) = self.list_state.selected(){
                        if let Some(VisibleItem::Submodule(data_idx)) = self.visible_map.get(visual_idx){
                            if let Some(item) = self.items.get_mut(*data_idx){
                                item.data.handle_input(key, &mut self.mode);
                                self.mode = InterfaceMode::Insert;
                            }
                        }
                    }
                }
                KeyCode::Char(' ') => self.next(),
                KeyCode::Backspace => self.prev(),
                KeyCode::Enter => self.handle_enter(),
                KeyCode::Char('f') => {
                    self.fullscreen = !self.fullscreen;
                }
                _ => {
                    if let Some(visual_idx) = self.list_state.selected(){
                        if let Some(VisibleItem::Submodule(data_idx)) = self.visible_map.get(visual_idx){
                            if let Some(item) = self.items.get_mut(*data_idx){
                                item.data.handle_input(key, &mut self.mode);
                            }
                        }
                    }
                }
            },
            InterfaceMode::Insert => {
                if key.code == KeyCode::Esc {
                if let Some(visual_idx) = self.list_state.selected() {
                    if let Some(VisibleItem::Submodule(data_idx)) = self.visible_map.get(visual_idx){
                        if let Some(item) = self.items.get_mut(*data_idx) {
                            item.data.handle_input(key, &mut self.mode);
                        }
                    }
                }
                    self.mode = InterfaceMode::Normal;
                    return;
                }
                if let Some(visual_idx) = self.list_state.selected() {
                    if let Some(VisibleItem::Submodule(data_idx)) = self.visible_map.get(visual_idx){
                        if let Some(item) = self.items.get_mut(*data_idx) {
                            item.data.handle_input(key, &mut self.mode);
                        }
                    }
                }
            }
        }
    }
}

mod major {
    use super::*;
    pub fn render(frame: &mut Frame, main: Rect, side: Rect, module: &mut Module, theme: &Theme){
        module.detailing(frame, main, theme);
    }
}

mod minor {
    use super::*;
    pub fn render(
        frame: &mut Frame, area: Rect, state: &mut ListState, items: &[ModuleItem], visible_map: &[VisibleItem], mode: &InterfaceMode, theme: &Theme) {
        let mut list_items = Vec::new();
        let (color, current_mode) = match mode {
            InterfaceMode::Insert => (theme.primary, "[I]"),
            InterfaceMode::Normal => (theme.secondary, "[N]"),
        };
        for (idx, item) in visible_map.iter().enumerate() {
            let is_selected = state.selected() == Some(idx);
            match item {
                VisibleItem::Header(cat) => {
                    let is_expanded = items
                        .iter()
                        .filter(|i| i.data.kind() == *cat)
                        .any(|i| i.is_expanded);
                    let icon = if is_expanded { "▼ " } else { "▶ " };
                    let style = if is_selected {
                        Style::new()
                            .fg(theme.secondary)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::new().fg(theme.dim)
                    };
                    list_items
                        .push(ListItem::new(format!("{}{}", icon, cat.as_str())).style(style));
                }
                VisibleItem::Submodule(sub_idx) => {
                    let item = &items[*sub_idx];
                    let sub_indices: Vec<usize> = items
                        .iter()
                        .enumerate()
                        .filter(|(_, i)| i.data.kind() == item.data.kind())
                        .map(|(idx, _)| idx)
                        .collect();
                    let is_last = sub_indices.last() == Some(sub_idx);
                    let prefix = if is_last { "  └─ " } else { "  ├─ " };
                    let style = if is_selected {
                        Style::new().fg(theme.text).bg(theme.selection)
                    } else {
                        Style::new().fg(theme.text)
                    };
                    list_items.push(
                        ListItem::new(format!("{}{}", prefix, item.data.submodname())).style(style),
                    );
                }
            }
        }
        let list_widget = List::new(list_items)
            .block(
                Block::bordered()
                    .title(format!("MODULES {}", current_mode))
                    .border_type(BorderType::Rounded)
                    .border_style(Style::new().fg(theme.border_active))
                    .style(Style::new().bg(theme.surface))
                    .title_alignment(Alignment::Left),
            )
            .highlight_style(Style::default().bg(theme.selection));
        frame.render_stateful_widget(list_widget, area, state);
    }
}

use crossterm::cursor::SetCursorStyle;
use crossterm::execute;
use std::io::stdout;

fn update_cursor_style(mode: &InterfaceMode) {
    match mode {
        InterfaceMode::Normal => {
            let _ = execute!(stdout(), SetCursorStyle::BlinkingBar);
        }
        InterfaceMode::Insert => {
            let _ = execute!(stdout(), SetCursorStyle::BlinkingBar);
        }
    }
}
