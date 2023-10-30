use std::fs;
use std::path;
use std::sync;
use serde::Deserialize;

const CONFIG_FILE: &str = "./freecam.toml";

static mut CONFIG: sync::OnceLock<Config> = sync::OnceLock::new();

#[derive(Clone, Default, Deserialize, Debug)]
pub struct Config {
    pub keybinds: Vec<ConfigKeybind>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ConfigKeybind {
    pub key: String,
    pub command: KeybindCommand,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(tag = "command")]
pub enum KeybindCommand {
    ToggleHUD,
    ToggleTimeControl,
    ToggleFreecam,
    SetTimeMultiplier {multiplier: f32},
}

pub fn get_config() -> Config {
    unsafe {
        CONFIG.get_or_init(|| read_config_file().unwrap_or_else(|| Config::default())).clone()
    }
}

fn read_config_file() -> Option<Config> {
    path::absolute(path::PathBuf::from(CONFIG_FILE))
        .map(|p| fs::read_to_string(p).ok()).ok()
        .flatten()
        .map(|f| toml::from_str(f.as_str()).ok())
        .flatten()
}
