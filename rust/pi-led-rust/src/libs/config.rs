use serde::Deserialize;
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use toml::value::Array;

// Thread safe static mutable variable that holds the configuration file contents (config.toml)
thread_local!(static CONFIG: RefCell<String> = RefCell::new(String::new()));

#[derive(Debug, Default, Deserialize)]
pub struct ConfigToml {
    // Tables in the config.topml file, e.g. [hardware]
    pub hardware: Hardware,
    pub effects: Effects,
}

#[derive(Debug, Default, Deserialize)]
pub struct Hardware {
    // Parameters that describe the hardware
    pub columns: i64,
    pub rows: i64,
    #[cfg(target_arch = "arm")]
    pub pin: i64,
    #[cfg(target_arch = "arm")]
    pub brightness: i64,
    pub simscale: i64,
    pub simupdatelimit: i64,
}

#[derive(Debug, Default, Deserialize)]
pub struct Effects {
    // Various parameters used by the effects
    pub playtime: i64,
    pub fontpath: String,
    pub imagepath: String,
    pub message: String,
    pub playlist: Array,
}

pub fn get_config() -> ConfigToml {
    CONFIG.with(|contents| {
        if contents.borrow().len() == 0 {
            // Read the config.toml file once and store the contents in the static storage
            let mut file = File::open("config.toml").expect("File not found!");
            file.read_to_string(&mut contents.borrow_mut())
                .expect("Error reading file!");
        }
    });
    let mut config_toml: ConfigToml = ConfigToml::default();

    // Return a deserialized pointer to the configuration file parameters, e.g. hardware.rows
    CONFIG.with(|contents| {
        config_toml = toml::from_str(&contents.borrow()).expect("Failed to deserialize Cargo.toml");
    });
    config_toml
}
