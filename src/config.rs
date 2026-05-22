use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config{
    #[serde(default = "def_walldir")]
    pub wallpaper_dir: PathBuf,
    #[serde(default = "def_scheme")]
    pub scheme_path: PathBuf,
    #[serde(default = "def_scheme_registry")]
    pub registry_path: PathBuf,
    #[serde(default = "def_socket")]
    pub daemon_socket: PathBuf,
    #[serde(default = "def_variant")]
    pub variant: String,
    #[serde(default = "def_wallwbin")]
    pub wallwbin: String,
    #[serde(default = "def_mouse")]
    pub mouse_on: bool
}

fn def_walldir() -> PathBuf{dirs::home_dir().unwrap().join(".local/share/wallpapers")}
fn def_scheme() -> PathBuf{dirs::home_dir().unwrap().join(".local/state/scheme.json")}
fn def_scheme_registry() -> PathBuf{dirs::home_dir().unwrap().join(".cache/wallwatch/wallcache/registry.json")}
fn def_socket() -> PathBuf{dirs::home_dir().unwrap().join("/tmp/scoutd/scoutd.sock")}
fn def_variant() -> String{"vibrant".to_string()}
fn def_wallwbin() -> String{"/usr/bin/wallwatch".to_string()}
fn def_mouse() -> bool{false}

impl Default for Config{
    fn default() -> Self{
        Self{
            wallpaper_dir: def_walldir(),
            scheme_path: def_scheme(),
            registry_path: def_scheme_registry(),
            daemon_socket: def_socket(),
            variant: def_variant(),
            wallwbin: def_wallwbin(),
            mouse_on: def_mouse()
        }
    }
}

impl Config{
    pub fn load() -> Self{
        let path = dirs::config_dir().unwrap().join("allio/config.toml");
        if path.exists(){
            let content = std::fs::read_to_string(path).unwrap();
            match toml::from_str(&content){
                Ok(config) => config,
                Err(e) => {panic!("\n❌ TOML Parsing Error in your config.toml!. {}", e)}
            }
        }else{Self::default()}
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(Config::load);
