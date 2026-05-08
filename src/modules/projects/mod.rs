pub mod wallwatch;
use crate::traits::{BaseModule, ProjectModule};

pub fn get_all() -> Vec<Box<dyn ProjectModule>>{
    vec![
        Box::new(wallwatch::Wallwatch::new()),
    ]
}
