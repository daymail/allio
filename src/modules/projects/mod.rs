pub mod wallwatch;
pub mod scoutd;
pub mod helpers;
pub mod backend;
use crate::traits::{ProjectModule};

pub fn get_all() -> Vec<Box<dyn ProjectModule>>{
    vec![
        Box::new(wallwatch::Wallwatch::new()),
        Box::new(scoutd::Scoutd::new())
    ]
}
