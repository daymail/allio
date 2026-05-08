use crate::traits::{ProjectModule, BaseModule};

pub struct Wallwatch{
    name: String,
}

impl Wallwatch{
    pub fn new()->Self{
        Self{
            name: "Wallwatch".to_string()
        }
    }
}

impl BaseModule for Wallwatch{
    fn submodname(&self) -> &str {
        "Wallwatch"
    }
    fn category(&self)-> &str {
        "Projects"
    }
}

impl ProjectModule for Wallwatch{
    fn name(&self)->&str {
        "Wallwatch"
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
