use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use toml::Table;

#[derive(Debug, Deserialize)]
struct Hardware {
    columns: i64,
    rows: i64,
    pin: i64,
    brightness: i64,
}

struct Effects {
    playtime: i64,
}

impl From<Table> for Hardware {
    fn from(value: Table) -> Self {
        Hardware {
            columns: value["hardware"]["columns"].as_integer().unwrap(),
            rows: value["hardware"]["rows"].as_integer().unwrap(),
            pin: value["hardware"]["pin"].as_integer().unwrap(),
            brightness: value["hardware"]["brightness"].as_integer().unwrap(),
        }
    }
}

impl From<Table> for Effects {
    fn from(value: Table) -> Self {
        Effects {
            playtime: value["effects"]["playtime"].as_integer().unwrap(),
        }
    }
}

pub fn get_config(key: &str) -> i64 {
    let mut file = File::open("config.toml").expect("File not found!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file!");
    let value = contents.parse::<Table>().unwrap();
    let hardware = Hardware::from(value.clone());
    let effects: Effects = Effects::from(value);
    let mut get_value: i64 = 0;
    match key {
        "columns" => get_value = hardware.columns,
        "rows" => get_value = hardware.rows,
        "pin" => get_value = hardware.pin,
        "brightness" => get_value = hardware.brightness,
        "playtime" => get_value = effects.playtime,
        _ => println!("Unknown config key {}", key),
    }
    get_value
}
