use crate::traits::{ProjectModule, BaseModule};

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
        "Scoutd"
    }
}

impl ProjectModule for Scoutd{
    fn name(&self)->&str{
        "Scoutd"
    }
    fn description(&self)->&str {
        "atomatic theming tool based on material3 (M3)"
    }
    fn git(&self)->String {
        todo!()
    }
    fn tree(&self)->Vec<String> {
        todo!()
    }
    fn project_root(&self)->std::path::PathBuf {
        todo!()
    }
}

