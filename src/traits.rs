//NOTE: This file js contains the defaults for each attachable module. it's traits!
use ratatui::{Frame,widgets::Paragraph,layout::{Rect}};
use crate::{
    theme::Theme,
    modules::interface::InterfaceMode
};
use crossterm::event::{KeyCode};
pub trait Component{
    fn name(&self) -> &str;
    fn id(&self) -> u16;
    fn is_active(&self) -> bool;
    fn set_active(&mut self, active: bool);
    fn event_handler(&mut self, key: crossterm::event::KeyEvent);
    fn render(&mut self, frame: &mut  Frame, area: Rect){
        let area = frame.area();
        let text = Paragraph::new("DEFAULT FRAME");
        frame.render_widget(text, area);
    }
}


//NOTE: module traits
pub trait BaseModule{
    fn category(&self)->crate::modules::ModuleCategory;// super-module category
    fn submodname(&self)-> &str;// submodule id name
    fn detailing(&mut self, frame: &mut Frame, area: Rect, theme: &Theme){
        frame.render_widget(Paragraph::new("DEFAULT RENDERER"),area);//FIXME: make this module specific. remove it
    }// rendering
}
pub trait ProjectModule: BaseModule{
    fn name(&self)->&str;
    fn description(&self)->&str;
    fn handle_input(&mut self, key: crossterm::event::KeyEvent, mode: &mut InterfaceMode);
    fn handle_mouse(&mut self, mouse: crossterm::event::MouseEvent, mode: InterfaceMode);
}
