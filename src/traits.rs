//NOTE: This file js contains the defaults for each attachable module. it's traits!
use ratatui::{Frame,widgets::Paragraph,layout::{Rect}};
use crossterm::event::{KeyCode};

pub trait Component{
    fn name(&self) -> &str;
    fn id(&self) -> u16;
    fn is_active(&self) -> bool;
    fn set_active(&mut self, active: bool);
    fn event_handler(&mut self, key: crossterm::event::KeyEvent){
        if !self.is_active(){return;}
        match key.code{
            KeyCode::Char('k')=>{todo!();}//move up
            KeyCode::Char('j')=>{todo!();}//move down
            KeyCode::Char('h')=>{todo!();}//move left
            KeyCode::Char('l')=>{todo!();}//move right
            KeyCode::Char('[')=>{todo!();}//move to the previous section in a section list.
            KeyCode::Char(']')=>{todo!();}//move to the next section in  a section list
            _ => {}
       }
    }
    fn render(&mut self, frame: &mut  Frame, area: Rect){
        let area = frame.size();
        let text = Paragraph::new("DEFAULT FRAME");
        frame.render_widget(text, area);
    }
}


//NOTE: module traits
pub trait BaseModule{
    fn submodname(&self) -> &str;
    fn category(&self)-> &str;
}
pub trait ProjectModule: BaseModule{
    fn name(&self)->&str;
    fn description(&self)->&str;
    fn git(&self)->String;
    fn tree(&self)->Vec<String>;
    fn project_root(&self)->std::path::PathBuf;
    //optional
    fn pid(&self)->Option<u32>{None}
    fn statistics(&self)->Option<(f64, f64)>{None}
}
