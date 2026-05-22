use crate::{
    config::CONFIG,
    icons::ICONS,
    modules::{projects::{backend::*, helpers::*}, interface::InterfaceMode},
    theme::Theme,
    traits::{BaseModule, ProjectModule},
};
use crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseButton, MouseEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    prelude::{Modifier, Span, Style},
    style::Color,
    text::Line,
    widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, Wrap},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WallwatchView{
    Overview,
    Palette
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegistryState{
    pub is_dark: bool,
    pub variant: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WallpaperItem{
    pub filename: String,
    pub relative_path: String,
    #[serde(rename = "seed_argb")]
    pub seed_argb: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegistryData{
    pub state: RegistryState,
    pub wallpapers: HashMap<String, WallpaperItem>
}

pub struct WallwatchStates {
    pub palette_state: ListState,
    pub input_buffer: String,
    pub cursor_position: usize,
    pub current_view: WallwatchView,
    pub wallpaper_list_state: ListState,
    pub wallpapers: Vec<(String, WallpaperItem)>,
    pub is_dark: bool,
    pub variant_override: Option<String>,

    //area bounds
    pub header_area: Rect,
    pub wallpaper_list_area: Rect,
    pub palette_list_area: Rect,
    pub control_panel_area: Rect,

    //fzf
    pub show_fzf: bool,
    pub fzf_query: String,
    pub fzf_matches: Vec<(usize, String, WallpaperItem)>,
    pub fzf_list_state: ListState
}

pub struct Wallwatch {
    name: String,
    states: WallwatchStates
}

impl Wallwatch {
    pub fn new() -> Self {
        let mut palette_state = ListState::default();
        palette_state.select(Some(0));
        let mut wallpaper_list_state = ListState::default();
        let registry_path = CONFIG.registry_path.clone();

        let mut wallpapers = Vec::new();
        let mut is_dark = true;
        let mut variant_override = None;
        if let Ok(content) = std::fs::read_to_string(&registry_path){
            if let Ok(registry_list) = serde_json::from_str::<Vec<RegistryData>>(&content){
                if let Some(first) = registry_list.first(){
                    is_dark = first.state.is_dark;
                    variant_override = Some(first.state.variant.clone());
                    wallpapers = first.wallpapers.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                }
            }else if let Ok(one_registry) = serde_json::from_str::<RegistryData>(&content){
                is_dark = one_registry.state.is_dark;
                variant_override = Some(one_registry.state.variant.clone());
                wallpapers = one_registry.wallpapers.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            }
        }
        wallpapers.sort_by(|a, b| a.1.filename.cmp(&b.1.filename));
        if !wallpapers.is_empty(){
            wallpaper_list_state.select(Some(0));
        }
        Self {
            name: "Wallwatch".to_string(),
            states: WallwatchStates {
                palette_state,
                wallpaper_list_state,
                wallpapers,
                is_dark,
                variant_override,
                input_buffer: String::new(),
                cursor_position: 0,
                current_view: WallwatchView::Overview,
                header_area: Rect::default(),
                wallpaper_list_area: Rect::default(),
                palette_list_area: Rect::default(),
                control_panel_area: Rect::default(),
                show_fzf: false,
                fzf_query: String::new(),
                fzf_matches: Vec::new(),
                fzf_list_state: ListState::default().with_selected(Some(0))
            },
        }
    }

    fn wallwatch_trigger(&self, idx: usize){
        if let Some((_, item)) = self.states.wallpapers.get(idx){
            let base_dir = &CONFIG.wallpaper_dir;
            let path = base_dir.join(&item.relative_path).to_string_lossy().into_owned();
            let variant = CONFIG.variant.trim().to_string();
            let mut args = vec!["-w".to_string(), path, "-V".to_string(), variant];
            if !self.states.is_dark{
                args.push("-l".to_string());
            }
            let wallwatch_bin = CONFIG.wallwbin.trim().to_string();
            std::thread::spawn(move ||{
                if let Ok(mut child) = std::process::Command::new(wallwatch_bin).args(args).spawn(){
                    let _ = child.wait();
                }
            });
        }
    }

    fn scroll_list(&mut self, forward: bool){
        if self.states.wallpapers.is_empty(){return;}
        let current = self.states.wallpaper_list_state.selected().unwrap_or(0);
        let next = if forward{
            (current + 1) % self.states.wallpapers.len()
        }else{
            if current == 0 {self.states.wallpapers.len() - 1} else{current - 1}
        };
        self.states.wallpaper_list_state.select(Some(next));
    }

}
impl ProjectModule for Wallwatch {
    fn name(&self) -> &str {
        "Wallwatch"
    }
    fn description(&self) -> &str {
        "atomatic theming tool based on material3 (M3)"
    }
    fn handle_input(&mut self, key: KeyEvent, mode: &mut InterfaceMode){
        if self.states.show_fzf{
            match key.code{
                KeyCode::Esc =>{
                    self.states.show_fzf = false;
                }
                KeyCode::Down =>{
                    if !self.states.fzf_matches.is_empty(){
                        let curr = self.states.fzf_list_state.selected().unwrap_or(0);
                        let nxt = (curr + 1) % self.states.fzf_matches.len();
                        self.states.fzf_list_state.select(Some(nxt));
                    }
                }
                KeyCode::Up =>{
                    if !self.states.fzf_matches.is_empty(){
                        let curr = self.states.fzf_list_state.selected().unwrap_or(0);
                        let prev = if curr == 0{self.states.fzf_matches.len() - 1} else{curr - 1};
                        self.states.fzf_list_state.select(Some(prev));
                    }
                }
                KeyCode::Char(c) =>{
                    self.states.fzf_query.push(c);
                    fzf::fzf_match_update(&mut self.states);
                }
                KeyCode::Backspace =>{
                    self.states.fzf_query.pop();
                    fzf::fzf_match_update(&mut self.states);
                }
                KeyCode::Enter =>{
                    if let Some(sel_idx) = self.states.fzf_list_state.selected(){
                        if let Some((original, _, _)) = self.states.fzf_matches.get(sel_idx){
                            let idx = *original;
                            self.states.wallpaper_list_state.select(Some(idx));
                            self.wallwatch_trigger(idx);
                        }
                        self.states.show_fzf = false;
                        *mode = InterfaceMode::Normal;
                    }
                }
                _ => {}
            }
            return;
        }
        match mode{
            InterfaceMode::Normal => {
                let mut selection_changed = false;
                match key.code{
                    KeyCode::Char('1') | KeyCode::Char('o') | KeyCode::Char('O') =>{
                        self.states.current_view = WallwatchView::Overview;
                    }
                    KeyCode::Char('2') | KeyCode::Char('p') | KeyCode::Char('P') =>{
                        self.states.current_view = WallwatchView::Palette;
                    }
                    KeyCode::Down | KeyCode::Char('j') =>{
                        if self.states.current_view == WallwatchView::Overview && !self.states.wallpapers.is_empty(){
                            self.scroll_list(true);
                            selection_changed = true;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') =>{
                        if self.states.current_view == WallwatchView::Overview && !self.states.wallpapers.is_empty(){
                            self.scroll_list(false);
                            selection_changed = true;
                        }
                    }
                    KeyCode::Char('t') => {
                        self.states.is_dark = !self.states.is_dark;
                        selection_changed = true;
                    }
                    KeyCode::Char('/') =>{
                        self.states.show_fzf = true;
                        self.states.fzf_query.clear();
                        fzf::fzf_match_update(&mut self.states);
                        self.states.fzf_list_state.select(Some(0));
                    }
                    _ => {}
                }
                if selection_changed && self.states.current_view == WallwatchView::Overview{
                    if let Some(idx) = self.states.wallpaper_list_state.selected(){
                        self.wallwatch_trigger(idx);
                    }
                }
            }
            InterfaceMode::Insert =>{
                match key.code {
                    KeyCode::Char(c) => {
                        self.states.input_buffer.push(c);
                        self.states.cursor_position += 1;
                    }
                    KeyCode::Backspace => {
                        self.states.input_buffer.pop();
                        self.states.cursor_position -= 1;
                    }
                    KeyCode::Enter => {
                        //FIXME: handle enter
                        self.states.input_buffer.clear();
                        self.states.cursor_position = 0;
                    }
                    _ => {}
                }
            }
        }
    }
    fn handle_mouse(&mut self, mouse: MouseEvent, mode: InterfaceMode){
        let pos = ratatui::layout::Position::new(mouse.column, mouse.row);
        if self.states.header_area.contains(pos){
            if let MouseEventKind::Down(MouseButton::Left) = mouse.kind{
                todo!()
            }
        }else if self.states.wallpaper_list_area.contains(pos){
            self.states.current_view = WallwatchView::Overview;
            let rel_row = mouse.row.saturating_sub(self.states.wallpaper_list_area.y + 1) as usize;
            let scroll_offset = self.states.wallpaper_list_state.offset();
            let abs_idx = rel_row + scroll_offset;
            match mouse.kind{
                MouseEventKind::Down(MouseButton::Left) => {
                    if abs_idx < self.states.wallpapers.len(){
                        self.states.wallpaper_list_state.select(Some(abs_idx));
                        self.wallwatch_trigger(abs_idx);
                    }
                }
                MouseEventKind::ScrollUp => self.scroll_list(false),
                MouseEventKind::ScrollDown => self.scroll_list(true),
                _ => {}
            }
        }else if self.states.palette_list_area.contains(pos){
            self.states.current_view = WallwatchView::Overview;
            let relative_y = mouse.row.saturating_sub(self.states.palette_list_area.y + 1) as usize;
            match mouse.kind{
                MouseEventKind::Down(MouseButton::Left) =>{todo!()} //TODO: stars/favourites that specific color, and is saved to generate a new scheme using it
                MouseEventKind::ScrollUp => {
                    let current = self.states.palette_state.selected().unwrap_or(0);
                    let prev = current.saturating_sub(1);
                    self.states.palette_state.select(Some(prev));
                }
                MouseEventKind::ScrollDown => {
                    let current = self.states.palette_state.selected().unwrap_or(0);
                    self.states.palette_state.select(Some(current + 1));
                }
                _ => {}
            }
        }else if self.states.control_panel_area.contains(pos){
            if let MouseEventKind::Down(MouseButton::Left) = mouse.kind{
                if mouse.row == self.states.control_panel_area.y + 1{
                    let text_field_label_offset = self.states.control_panel_area.x + 15;
                    if mouse.column >= text_field_label_offset{
                        let target_idx = (mouse.column - text_field_label_offset) as usize;
                        self.states.cursor_position = target_idx.min(self.states.input_buffer.len());
                    }
                }
            }
        }
    }
}

impl BaseModule for Wallwatch {
    fn category(&self) -> crate::modules::ModuleCategory {
        crate::modules::ModuleCategory::Projects
    }
    fn submodname(&self) -> &str {
        &self.name
    }
    fn detailing(&mut self, frame: &mut Frame, area: Rect, theme: &Theme){
        let [header, body] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .areas(area);
        self.states.header_area = header;
        let mock_data = ProjectDefinition {
            name: self.submodname().to_string(),
            description: self.description().to_string(),
            git_enabled: true,
            daemon_running: process_running("scoutd"),
            process: process_running("wallwatch"),
        };
        let header_config = HeaderConfig {
            name: self.submodname(),
            description: self.description(),
            indicators: mock_data.get_status_indicators(),
        };
        header::render_header(frame, header, self.states.current_view, header_config, theme);
        let [right, palette_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Percentage(30)])
            .areas(body);
        let [wallpaper_list, control_panel] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Fill(1)])
            .areas(right);

        self.states.palette_list_area = palette_area;
        self.states.wallpaper_list_area = wallpaper_list;
        self.states.control_panel_area = control_panel;
        palette::render_palette(frame, palette_area, &mut self.states.palette_state, theme);
        let mut temp_list_state = self.states.wallpaper_list_state.clone();
        main::render_main(frame, right, &self.states, &mut temp_list_state, theme);
        self.states.wallpaper_list_state = temp_list_state;

        if self.states.show_fzf{
            let mut temp_fzf_state = self.states.fzf_list_state.clone();
            fzf::render_fzf(frame, area, &self.states, &mut temp_fzf_state, theme);
            self.states.fzf_list_state = temp_fzf_state;
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemeData {
    pub filename: String,
    pub theme: String,
    pub variant: String,
    pub hash: String,
    pub colors: HashMap<String, String>,
}

mod palette {
    //NOTE: split this vertically to accommodate space for file info from scheme.json (small)
    use super::*;
    pub fn render_palette(frame: &mut Frame, area: Rect, state: &mut ListState, theme: &Theme){
        let [palette, info] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Percentage(20)])
            .areas(area);
        let home = get_env("HOME");
        let path = format!("{}/.local/state/scheme.json", home); //NOTE: this will be in details

        let scheme_list = match load_data(&path) {
            Ok(data) => data,
            Err(e) => {
                frame.render_widget(Paragraph::new(format!("JSON Error: {}", e)), area);
                return;
            }
        };
        let mut colors: Vec<_> = scheme_list.colors.iter().collect();
        colors.sort_by_key(|(name, _)| *name);
        let items: Vec<ListItem> = colors
            .iter()
            .map(|(name, hex)| {
                let color = parse_hex(hex);
                let left_side = format!("{} {}", ICONS.square, name.to_lowercase());
                let inner_width = palette.width.saturating_sub(4) as usize;
                let filler_len = inner_width
                    .saturating_sub(left_side.len())
                    .saturating_sub(7);
                let filler = " ".repeat(filler_len);
                let line = Line::from(vec![
                    Span::styled(format!(" {} ", ICONS.square), Style::default().fg(color)),
                    Span::styled(format!(" {}", name.to_lowercase()), Style::default()),
                    Span::raw(filler),
                    Span::styled(
                        hex.to_string(),
                        Style::default()
                            .fg(theme.dim)
                            .add_modifier(Modifier::ITALIC),
                    ),
                ]);
                ListItem::new(line)
            })
            .collect();
        let palette_list = List::new(items)
            .block(Block::bordered())
            .highlight_style(Style::default().bg(theme.selection))
            .highlight_symbol("> ");
        frame.render_stateful_widget(palette_list, palette, state);

        //TODO: info section
        let info_text = vec![
            Line::from(vec![
                Span::styled("theme: ", Style::default().fg(theme.secondary)),
                Span::styled(&scheme_list.theme, Style::default().fg(theme.dim)),
            ]),
            Line::from(vec![
                Span::styled("Variant: ", Style::default().fg(theme.secondary)),
                Span::styled(&scheme_list.variant, Style::default().fg(theme.dim)),
            ]),
            Line::from(vec![
                Span::styled("File: ", Style::default().fg(theme.secondary)),
                Span::styled(&scheme_list.filename, Style::default().fg(theme.dim)),
            ]),
            Line::from(vec![
                Span::styled("Id: ", Style::default().fg(theme.secondary)),
                Span::styled(&scheme_list.hash, Style::default().fg(theme.dim)),
            ]),
            Line::from(vec![
                Span::styled("Path: ", Style::default().fg(theme.secondary)),
                Span::styled(&path, Style::default().fg(theme.dim)),
            ]),
        ];
        frame.render_widget(
            Paragraph::new(info_text)
                .block(
                    Block::bordered()
                        .title("Scheme Info")
                        .border_type(BorderType::Rounded),
                )
                .wrap(Wrap { trim: true }),
            info,
        )
    }
    fn load_data(path: &str) -> Result<SchemeData, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let scheme: SchemeData = serde_json::from_str(&content)?;
        Ok(scheme)
    }
    fn parse_hex(hex: &str) -> Color {
        let hex = hex.trim_start_matches('#');
        if let Ok(rgb) = u32::from_str_radix(hex, 16) {
            let r = ((rgb >> 16) & 0xFF) as u8;
            let g = ((rgb >> 8) & 0xFF) as u8;
            let b = (rgb & 0xFF) as u8;
            Color::Rgb(r, g, b)
        } else {
            Color::White
        }
    }
}

mod main {
    use super::*;
    pub fn render_main(frame: &mut Frame, area: Rect, states: &WallwatchStates ,list_state: &mut ListState , theme: &Theme){
        let [wallpaper_list, control_panel] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Fill(1)])
            .areas(area);
        let items: Vec<ListItem> = states.wallpapers.iter().map(|(_, item)|{
            let clean_hex = item.seed_argb.to_uppercase().replace("0X", "") .replace("0[", "") .replace("0]", "");
            let (r,g,b) = if clean_hex.len() >= 6{
                let color = &clean_hex[clean_hex.len() - 6..];
                if let Ok(rgb) = u32::from_str_radix(color, 16){
                    (((rgb >> 16) & 0xFF) as u8, ((rgb >> 8) & 0xFF) as u8, (rgb & 0xFF) as u8)
                }else{(255, 255, 255)}
            }else{(255, 255, 255)};

            let scale_color = |r: u8, g:u8, b: u8, factor: f32| -> Color{
                Color::Rgb(
                    ((r as f32) * factor).clamp(0.0, 255.0) as u8,
                    ((g as f32) * factor).clamp(0.0, 255.0) as u8,
                    ((b as f32) * factor).clamp(0.0, 255.0) as u8
                )
            };

            let sym_5 = scale_color(r,g, b, 0.4);
            let sym_4 = scale_color(r,g, b, 0.7);
            let sym_3 = Color::Rgb(r,g,b);
            let sym_2 = scale_color(r,g, b, 1.3);
            let sym_1 = scale_color(r,g, b, 1.6);
            let sym_gradient = [sym_1, sym_2, sym_3, sym_4, sym_5];
            let disp_name = format!("{}", item.filename);

            let mut sym_span: Vec<Span> = sym_gradient.iter().enumerate().map(|(i, &color)|{
                let symbol = if i < 4 {format!("{} ", ICONS.star)} else{ICONS.star.to_string()};
                Span::styled(symbol, Style::default().fg(color))
            }).collect();

            let inner_width = wallpaper_list.width.saturating_sub(4) as usize;
            let filler_len = inner_width.saturating_sub(disp_name.len()).saturating_sub(10);
            let filler = " ".repeat(filler_len);

            let mut line_span = vec![
                Span::styled(disp_name, Style::default().fg(theme.text)),
                Span::raw(filler),
            ];
            line_span.append(&mut sym_span);
            ListItem::new(Line::from(line_span))
        }).collect();

        let current_theme_icon = if states.is_dark {ICONS.moon}else{ICONS.sun};
        let theme_color = if states.is_dark{theme.primary}else{theme.fg};
        let active_variant = if CONFIG.variant != "vibrant"{
            &CONFIG.variant
        }else{
            states.variant_override.as_deref().unwrap_or(&CONFIG.variant)
        };
        let right_title_content = Line::from(vec![
            Span::styled(format!("{} ", active_variant), Style::default().fg(theme.primary)),
            Span::styled(format!("{}", current_theme_icon), Style::default().fg(theme_color))
        ]);
        let left_title = Line::from("Wallpaper Repo").alignment(Alignment::Left);
        let right_title = Line::from(right_title_content).alignment(Alignment::Right);

        let list_widget = List::new(items)
            .block(
                Block::bordered()
                    .title(left_title)
                    .title(right_title)
                    .border_style(Style::default().fg(theme.border_active)))
            .highlight_style(Style::default().bg(theme.selection).fg(theme.primary))
            .highlight_symbol("*");
        frame.render_stateful_widget(list_widget, wallpaper_list, list_state);
        control_panel::render_control_panel(frame, control_panel, theme);
    }
}

mod header{
    use super::*;
    pub fn render_header(frame: &mut Frame, area: Rect, view: WallwatchView, config: HeaderConfig, theme: &Theme){
        let mut tab_span = Vec::new();
        tab_span.push(Span::raw(" "));

        let ov_color = if view == WallwatchView::Overview {theme.border_active} else {theme.border_inactive};
        let ov_mod = if view == WallwatchView::Overview {Modifier::BOLD} else {Modifier::DIM};
        tab_span.push(Span::styled("[1] ", Style::default().fg(theme.primary).add_modifier(ov_mod)));
        tab_span.push(Span::styled("Overview  ", Style::default().fg(ov_color).add_modifier(ov_mod)));


        let pal_color = if view == WallwatchView::Palette {theme.border_active} else {theme.border_inactive};
        let pal_mod = if view == WallwatchView::Palette {Modifier::BOLD} else {Modifier::DIM};
        tab_span.push(Span::styled("[2] ", Style::default().fg(theme.primary).add_modifier(pal_mod)));
        tab_span.push(Span::styled("Wallpaperview ", Style::default().fg(pal_color).add_modifier(pal_mod)));

        let title = Span::styled(format!("{}", config.name.to_uppercase()), Style::default().bg(theme.secondary).fg(theme.border_inactive));
        let description = Span::styled(config.description, Style::default().fg(theme.dim).add_modifier(Modifier::ITALIC | Modifier::DIM),);
        let block = Block::bordered()
            .title(Line::from(tab_span).alignment(Alignment::Left))
            .title(Line::from(title).alignment(Alignment::Right))
            .title_alignment(Alignment::Right)
            .border_style(Style::default().fg(theme.border_active));
        frame.render_widget(Paragraph::new(description)
            .block(block)
            .alignment(Alignment::Right), area);
    }
}

mod control_panel{
    use super::*;
    pub fn render_control_panel(frame: &mut Frame, area: Rect, theme: &Theme){
        let cp_title = Line::from("Control-Panel").alignment(Alignment::Left);
        let block = Block::bordered().title(cp_title).border_style(Style::default().fg(theme.border_active));
        let inner_area = block.inner(area);
        frame.render_widget(block, area);
    }
}

mod fzf{
    use super::*;
    pub fn fzf_match_update(states: &mut WallwatchStates){
        let query = states.fzf_query.to_lowercase();
        states.fzf_matches = states.wallpapers.iter().enumerate().filter(|(_, (_, item))|{
            if query.is_empty(){
                true
            }else{
                item.relative_path.to_lowercase().contains(&query)
            }
        }).map(|(idx, (k, v))| (idx, k.clone(), v.clone())).collect();
        let current_sel = states.fzf_list_state.selected().unwrap_or(0);
        if states.fzf_matches.is_empty(){
            states.fzf_list_state.select(None)
        }else if current_sel >= states.fzf_matches.len(){
            states.fzf_list_state.select(Some(states.fzf_matches.len() - 1))
        }else{
            states.fzf_list_state.select(Some(current_sel))
        }
    }

    pub fn render_fzf(frame: &mut Frame, area: Rect, states: &WallwatchStates, list_state: &mut ListState, theme: &Theme){
        let modal_area = centered_rect(50, 40, area);
        let [results_area, prompt_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)]).areas(modal_area);
        frame.render_widget(ratatui::widgets::Clear, modal_area);

        let query = states.fzf_query.to_lowercase();
        let list_items: Vec<ListItem> = states.fzf_matches.iter().map(|(_,_,item)|{
            let full_path = item.relative_path.clone();
            let full_path_lower = item.relative_path.to_lowercase();
            let mut line_spans = vec![
                Span::styled(format!("{} ", ICONS.star), Style::default().fg(theme.primary)),
            ];

            if !query.is_empty(){
                if let Some(start_idx) = full_path_lower.find(&query){
                    let end_idx = start_idx + query.len();
                    let prefix = &full_path[..start_idx];
                    let match_part = &full_path[start_idx..end_idx];
                    let suffix = &full_path[end_idx..];

                    if !prefix.is_empty(){
                        line_spans.push(Span::styled(prefix.to_string(), theme.text));
                    }
                    line_spans.push(Span::styled(match_part.to_string(), Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)));
                    if !suffix.is_empty(){
                        line_spans.push(Span::styled(suffix.to_string(), theme.text));
                    }
                }else{
                    line_spans.push(Span::styled(full_path, theme.text));
                }
            }else{
                line_spans.push(Span::raw(full_path));
            }
            ListItem::new(Line::from(line_spans))
        }).collect();

        let tt_count = states.wallpapers.len();
        let match_count = states.fzf_matches.len();
        let title = Line::from(format!("Current Buffer Fuzzy ({}/{})", match_count, tt_count)).alignment(Alignment::Center);
        let result_widget = List::new(list_items)
            .block(
                Block::bordered()
                    .title(title)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(theme.border_active))
                    .style(Style::default().bg(theme.surface))
            )
                .style(theme.text)
                .highlight_style(Style::default().bg(theme.selection));
        frame.render_stateful_widget(result_widget, results_area, list_state);
        let prompt_widget = Paragraph::new(Line::from(vec![
            Span::styled("> ", Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD)),
            Span::styled(states.fzf_query.clone(), theme.text)
        ])).block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(theme.primary))
                    .style(Style::default().bg(theme.surface))
            );
        frame.render_widget(prompt_widget, prompt_area);
        let cursor_x = prompt_area.x + 3 + states.fzf_query.len() as u16;
        let cursor_y = prompt_area.y + 1;
        frame.set_cursor_position((cursor_x, cursor_y));
    }
}

mod help{
    use super::*;
    pub fn render_help_menu(frame: &mut Frame, area: Rect){
        let floating_area = centered_rect(40, 60, area);
    }
}
