use crate::{
    icons::ICONS,
    modules::projects::{backend::*, helpers::*},
    theme::PALETTE,
    traits::{BaseModule, ProjectModule},
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Modifier, Span, Style},
    style::Color,
    text::Line,
    widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, Wrap},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct WallwatchStates {
    pub palette_state: ListState,
    pub input_buffer: String,
    pub cursor_position: usize,
}

pub struct Wallwatch {
    name: String,
    states: WallwatchStates,
}

impl Wallwatch {
    pub fn new() -> Self {
        let mut palette_state = ListState::default();
        palette_state.select(Some(0));
        Self {
            name: "Wallwatch".to_string(),
            states: WallwatchStates {
                palette_state,
                input_buffer: String::new(),
                cursor_position: 0,
            },
        }
    }
}

impl ProjectModule for Wallwatch {
    fn name(&self) -> &str {
        "Wallwatch"
    }
    fn description(&self) -> &str {
        "atomatic theming tool based on material3 (M3)"
    }
    fn handle_input(&mut self, key: KeyEvent) {
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
                //handle key inputs
                self.states.input_buffer.clear();
                self.states.cursor_position = 0;
            }
            _ => {}
        }
    }
}

impl BaseModule for Wallwatch {
    fn category(&self) -> crate::modules::ModuleCategory {
        crate::modules::ModuleCategory::Projects
    }
    fn submodname(&self) -> &str {
        "Wallwatch"
    }
    fn detailing(&mut self, frame: &mut Frame, area: Rect) {
        let [header, body] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .areas(area);
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
        render_header(frame, header, header_config);
        let [right, palette_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Percentage(30)])
            .areas(body);
        palette::render_palette(frame, palette_area, &mut self.states.palette_state);
        main::render_main(frame, right, &self.states);
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
    pub fn render_palette(frame: &mut Frame, area: Rect, state: &mut ListState) {
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
                            .fg(PALETTE.dim)
                            .add_modifier(Modifier::ITALIC),
                    ),
                ]);
                ListItem::new(line)
            })
            .collect();
        let palette_list = List::new(items)
            .block(Block::bordered())
            .highlight_style(Style::default().bg(PALETTE.selection))
            .highlight_symbol("> ");
        frame.render_stateful_widget(palette_list, palette, state);

        //TODO: info section
        let info_text = vec![
            Line::from(vec![
                Span::styled("Theme: ", Style::default().fg(PALETTE.secondary)),
                Span::styled(&scheme_list.theme, Style::default().fg(PALETTE.dim)),
            ]),
            Line::from(vec![
                Span::styled("Variant: ", Style::default().fg(PALETTE.secondary)),
                Span::styled(&scheme_list.variant, Style::default().fg(PALETTE.dim)),
            ]),
            Line::from(vec![
                Span::styled("File: ", Style::default().fg(PALETTE.secondary)),
                Span::styled(&scheme_list.filename, Style::default().fg(PALETTE.dim)),
            ]),
            Line::from(vec![
                Span::styled("Hash: ", Style::default().fg(PALETTE.secondary)),
                Span::styled(&scheme_list.hash, Style::default().fg(PALETTE.dim)),
            ]),
            Line::from(vec![
                Span::styled("Path: ", Style::default().fg(PALETTE.secondary)),
                Span::styled(&path, Style::default().fg(PALETTE.dim)),
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
    pub fn render_main(frame: &mut Frame, area: Rect, states: &WallwatchStates) {
        let [cli_area, plotter] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .areas(area);
        let [cli, variant] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Percentage(15)])
            .areas(cli_area);
        let cli_block = Block::bordered()
            .title("cli")
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(PALETTE.secondary));
        let max_text_len = cli.width.saturating_sub(6) as usize;
        let display_text = if states.input_buffer.len() > max_text_len {
            let start = states.input_buffer.len() - max_text_len;
            &states.input_buffer[start..]
        } else {
            &states.input_buffer
        };

        let input_text = format!(" >> {}", display_text);
        frame.render_widget(Paragraph::new(input_text).block(cli_block), cli);
        let virtual_cursor_pos = std::cmp::min(states.cursor_position, max_text_len);
        let cursor_x = cli.x + 5 + virtual_cursor_pos as u16;
        let cursor_y = cli.y + 1;
        frame.set_cursor_position((cursor_x, cursor_y));
    }
}
