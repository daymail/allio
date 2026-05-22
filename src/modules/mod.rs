pub mod interface;
pub mod projects;
pub mod cli;
use ratatui::{Frame, layout::Rect};
use crate::{
    theme::Theme,
    traits::{ProjectModule},
    modules::interface::InterfaceMode
};

pub enum Module{
    Project(Box<dyn ProjectModule>),
    //TODO: add other modules here
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ModuleCategory {
    Projects,
    Tools,
    Games,
}

impl ModuleCategory{
    pub fn as_str(&self)-> &'static str{
        match self{
            Self::Projects => "Projects",
            Self::Tools=> "Tools",
            Self::Games=> "Games"
        }
    }
}

impl Module{
    pub fn kind(&self)->ModuleCategory{
        match self{
            Module::Project(p)=> p.category()
        }
    }
    pub fn submodname(&self)->&str{
        match self{
            Module::Project(p) => p.submodname()
        }
    }
    pub fn detailing(&mut self, frame: &mut Frame, area: Rect, theme: &Theme){
        match self{
            Module::Project(p) => p.detailing(frame, area, theme),
        }
    }
    pub fn handle_input(&mut self, key: crossterm::event::KeyEvent, mode: &mut InterfaceMode){
        match self{
            Module::Project(p) => p.handle_input(key, mode)
        }
    }
    pub fn handle_mouse(&mut self, mouse: crossterm::event::MouseEvent, mode: InterfaceMode){
        match self{
            Module::Project(p) => p.handle_mouse(mouse, mode)
        }
    }
}

pub fn get_all_modules()->Vec<Module>{
    let mut all = Vec::new();
    for p in projects::get_all(){
        all.push(Module::Project(p));
    }
    all
}
