use crate::component::Component;
use ratatui::{layout, Frame};

pub struct WallwatchModule{
    pub active: bool,
}

impl Component for WallwatchModule{
    fn name(&self) -> &str {"Wallwatch"}
    fn id(&self)-> u32 {1}
    fn is_active(&self)->bool{self.active}
    fn set_active(&mut self, active: bool){self.active=active}
    fn render(&mut self, frame: &mut Frame, area: layout::Rect){todo!();}
}
