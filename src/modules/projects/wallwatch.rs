use std::collections::HashMap;
use crate::{
    icons::ICONS,
    theme::PALETTE,
    traits::{ProjectModule, BaseModule},
    modules::projects::{backend::*, helpers::*}
};
use serde::{Deserialize, Serialize};
use ratatui::{
    prelude::{Span, Modifier, Style},
    widgets::{Block, BorderType, ListState, Paragraph, ListItem, List},
    layout::{Direction, Layout, Rect, Constraint},
    style::Color,
    text::Line,
    Frame
};

pub struct WallwatchStates{
    pub palette_state: ListState,
}

pub struct Wallwatch{
    name: String,
    states: WallwatchStates
}

impl Wallwatch{
    pub fn new()->Self{
        let mut palette_state = ListState::default();
        palette_state.select(Some(0));
        Self{
            name: "Wallwatch".to_string(),
            states: WallwatchStates{
                palette_state
            }
        }
    }
}

impl ProjectModule for Wallwatch{
    fn name(&self)->&str {
        "Wallwatch"
    }
    fn description(&self)->&str {
        "atomatic theming tool based on material3 (M3)"
    }
}

impl BaseModule for Wallwatch{
    fn category(&self) -> crate::modules::ModuleCategory{
        crate::modules::ModuleCategory::Projects
    }
    fn submodname(&self)-> &str {
        "Wallwatch"
    }
    fn detailing(&mut self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
        let [header, body] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Fill(1)
            ]).areas(area);
        let mock_data = ProjectDefinition{
            name: self.submodname().to_string(),
            description: self.description().to_string(),
            git_enabled: true,
            daemon_running: process_running("scoutd"),
            process: process_running("wallwatch")
        };
        let header_config = HeaderConfig{
            name: self.submodname(),
            description: self.description(),
            indicators: mock_data.get_status_indicators()
        };
        render_header(frame, header, header_config);
        let [right, palette_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),
                Constraint::Percentage(30)
            ]).areas(body);
        palette::render_palette(frame, palette_area, &mut self.states.palette_state);
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all= "camelCase")]
pub struct SchemeData{
    pub filename: String,
    pub theme: String,
    pub variant: String,
    pub hash: String,
    pub colors: HashMap<String, String>
}

mod palette{//NOTE: split this vertically to accommodate space for file info from scheme.json (small)
    use super::*;
    pub fn render_palette(frame: &mut Frame, area: Rect, state: &mut ListState){
        let [palette, info] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Percentage(20)
            ]).areas(area);
        let home = get_env("HOME");
        let path = format!("{}/.local/state/scheme.json", home); //NOTE: this will be in details

        let scheme_list = match load_data(&path){
            Ok(data) => data,
            Err(e) =>{
                frame.render_widget(Paragraph::new(format!("JSON Error: {}", e)), area);
                return;
            }
        };
        let mut colors: Vec<_> = scheme_list.colors.iter().collect();
        colors.sort_by_key(|(name, _)| *name);
        let items: Vec<ListItem> = colors.iter().map(|(name, hex)|{
            let color = parse_hex(hex);
            let left_side = format!("{} {}", ICONS.square, name.to_lowercase());
            let inner_width = palette.width.saturating_sub(4) as usize;
            let filler_len = inner_width.saturating_sub(left_side.len()).saturating_sub(7);
            let filler = " ".repeat(filler_len);
            let line = Line::from(vec![
                Span::styled(format!(" {} ", ICONS.square), Style::default().fg(color)),
                Span::styled(format!(" {}", name.to_lowercase()), Style::default()),
                Span::raw(filler),
                Span::styled(hex.to_string(), Style::default().fg(PALETTE.dim).add_modifier(Modifier::ITALIC))
            ]);
            ListItem::new(line)
        }).collect();
        let palette_list = List::new(items)
            .block(Block::bordered())
            .highlight_style(Style::default().bg(PALETTE.selection))
            .highlight_symbol("> ");
        frame.render_stateful_widget(palette_list, palette, state);

        //TODO: info section
        frame.render_widget(Block::bordered(), info);
    }
    fn load_data(path: &str) -> Result<SchemeData, Box<dyn std::error::Error>>{
        let content = std::fs::read_to_string(path)?;
        let scheme: SchemeData = serde_json::from_str(&content)?;
        Ok(scheme)
    }
    fn parse_hex(hex: &str)->Color{
        let hex = hex.trim_start_matches('#');
        if let Ok(rgb) = u32::from_str_radix(hex, 16){
            let r = ((rgb >> 16) & 0xFF) as u8;
            let g = ((rgb >> 8) & 0xFF) as u8;
            let b = (rgb & 0xFF) as u8;
            Color::Rgb(r,g,b)
        }else{
            Color::White
        }
    }
}
