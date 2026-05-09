pub mod interface;
pub mod projects;
use crate::traits::{ProjectModule};

pub enum Module{
    Project(Box<dyn ProjectModule>),
    //TODO: add other modules here
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ModuleCategory {
    Projects,
    Tools,
    System,
    Games,
}

impl ModuleCategory{
    pub fn as_str(&self)-> &'static str{
        match self{
            Self::Projects => "Projects",
            Self::Tools=> "Tools",
            Self::System=> "System",
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
}

pub fn get_all_modules()->Vec<Module>{
    let mut all = Vec::new();
    for p in projects::get_all(){
        all.push(Module::Project(p));
    }
    all
}
