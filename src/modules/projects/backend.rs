use std::env;
use sysinfo::System;
pub struct ProjectDefinition{
    pub name: String, // project/ submodule name
    pub description: String, // brief description
    pub git_enabled: bool, // if it has git
    pub daemon_running: bool, // if the daemon is running for IPC. (scoutd)
    pub process: bool, //if it's a process or not
}

impl ProjectDefinition{
    pub fn get_status_indicators(&self)-> Vec<(&str, bool)>{
        vec![
            (" GIT", self.git_enabled),
            ("󰊠 DAEMON", self.daemon_running),
            (" PROC", self.process)
        ]
    }
}

pub fn process_running(proc: &str)->bool{
    let mut sys = System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, false);
    sys.processes().values().any(|process| process.name().to_string_lossy() == proc)
}

pub fn get_env(var: &str)->String{
    env::var(var).expect(&format!("{} environment variable not set", var))
}
