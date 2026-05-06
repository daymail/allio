use crate::component::Component;
pub struct App{
    pub modules: Vec<Box<dyn Component>>,
    pub active_module_index: usize,
    pub should_quit: bool
}

impl App{
    pub fn module_switch(&mut self, new_index: usize){
        if new_index < self.modules.len(){
            self.modules[self.active_module_index].set_active(false);
            self.active_module_index = new_index;
            self.modules[self.active_module_index].set_active(true);
        }
    }
}
