use serde::Deserialize;
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use toml::value::Array;

thread_local!(static CONFIG: RefCell<String> = RefCell::new(String::new()));

#[derive(Debug, Default, Deserialize)]
pub struct ConfigToml {
    pub hardware: Hardware,
    pub effects: Effects,
}

#[derive(Debug, Default, Deserialize)]
pub struct Hardware {
    pub columns: i64,
    pub rows: i64,
    pub pin: i64,
    pub brightness: i64,
    pub simscale: i64,
}

#[derive(Debug, Default, Deserialize)]
pub struct Effects {
    pub playtime: i64,
    pub fontpath: String,
    pub message: String,
    pub playlist: Array,
}

pub fn get_config() -> ConfigToml {
    CONFIG.with(|contents| {
        if contents.borrow().len() == 0 {
            let mut file = File::open("config.toml").expect("File not found!");
            file.read_to_string(&mut contents.borrow_mut())
                .expect("Error reading file!");
        }
    });
    let mut config_toml: ConfigToml = ConfigToml::default();
    CONFIG.with(|contents| {
        config_toml = toml::from_str(&contents.borrow()).expect("Failed to deserialize Cargo.toml");
    });
    config_toml
}
