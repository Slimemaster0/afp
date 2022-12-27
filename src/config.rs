use std::path::PathBuf;
use crate::{ APP_NAME, CONFIG_FILE };
use std::fs::File;
use std::io::prelude::*;
use serde::{ Serialize, Deserialize };

// config struct
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub logo: String,
    pub items: Vec<String>,
}

// --- Configuration ---
// --- open config file ---
fn open_config_file(cf: &mut PathBuf) -> String { // cf stands for config file
    cf.push(APP_NAME);
    cf.push(CONFIG_FILE);

    let mut syscf = PathBuf::from("/etc");
    syscf.push(APP_NAME);
    syscf.push(CONFIG_FILE);
    
    match File::open(cf) {
        Ok(f) => {
            let mut file: File = f;
            let mut config_file: String = String::new();
            file.read_to_string(&mut config_file).expect("Err: could not read config.json");
            return config_file;
        },

        Err(_) => {
            match File::open(syscf) { // try system wide config file
                Ok(f) => {
                    let mut file: File = f;
                    let mut config_file: String = String::new();
                    file.read_to_string(&mut config_file).expect("Err: could not read config.json");
                    return config_file;
                }
                Err(_) => { // Default config
                    let config_string: String = r#"
{
    "logo": "auto",
    "items": [
        "user host",
        "session type",
        "distro",
        "kernel",
        "device",
        "vendor",
        "ram",
        "editor",
        "shell",
        "cpu",
        "de"
    ]
}
"#.to_string();
                    return config_string;
                }
            }
        }
    }
}

pub fn read_config(mut cf: &mut PathBuf) -> Config {
    let contents_string: String = open_config_file(&mut cf);
    let contents_str: &str = &contents_string; // Converts contents_string to &str

    let config: Config = serde_json::from_str(contents_str).expect("Err: Could not parse config.json");
    return config;
}
