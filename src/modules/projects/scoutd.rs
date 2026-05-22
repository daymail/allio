use crate::{
    theme::Theme,
    traits::{ProjectModule, BaseModule},
    modules::{projects::{backend::*, helpers::*}, interface::InterfaceMode}
};

pub struct Scoutd{
    name: String,
}

impl Scoutd{
    pub fn new()->Self{
        Self{
            name: "Scoutd".to_string()
        }
    }
}

impl BaseModule for Scoutd{
    fn category(&self)->crate::modules::ModuleCategory {
        crate::modules::ModuleCategory::Projects
    }
    fn submodname(&self)-> &str {
        &self.name
    }
    fn detailing(&mut self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect, theme: &Theme){
        let mock_data = ProjectDefinition{
            name: self.submodname().to_string(),
            description: self.description().to_string(),
            git_enabled: true,
            daemon_running: process_running("scoutd"),
            process: process_running("scoutd")
        };
        let header_config = HeaderConfig{
            name: self.submodname(),
            description: self.description(),
            indicators: mock_data.get_status_indicators()
        };
        render_header(frame, area, header_config, theme);
    }
}

impl ProjectModule for Scoutd{
    fn name(&self)->&str{
        "Scoutd"
    }
    fn description(&self)->&str {
        "Scout-d (daemon) for IPC communication via DBus"
    }
    fn handle_input(&mut self, key: crossterm::event::KeyEvent, mode: &mut InterfaceMode){
        todo!()
    }
    fn handle_mouse(&mut self, mouse: crossterm::event::MouseEvent, mode: crate::modules::interface::InterfaceMode) {
        todo!()
    }
}

