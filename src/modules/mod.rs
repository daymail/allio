pub mod interface;
pub mod projects;
use crate::modules::interface::Module;

pub fn get_all_modules()->Vec<Module>{
    let mut all = Vec::new();
    for p in projects::get_all(){
        all.push(Module::Project(p));
    }
    all
}
