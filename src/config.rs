use std::path::PathBuf;
use crate::{ APP_NAME, CONFIG_FILE };
use std::fs::File;
use std::io::prelude::*;
use serde::{ Serialize, Deserialize };

// printmodule struct
#[derive(Serialize, Deserialize)]
pub struct PrintMod {
    pub mod_type: String, // Module type. like a builtin module or an Environment variable.
    pub mod_name: String, // The name of the module, the name of the builtin module or environment variable.
    pub mod_title: String,
}

// config struct
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub logo: String,
    pub items: Vec<PrintMod>,
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
        {
            "mod_type": "builtin mod",
            "mod_name": "user host",
            "mod_title": ""
        },
        {
            "mod_type": "env_var",
            "mod_name": "XDG_SESSION_TYPE",
            "mod_title": "Session Type: "
        },
        {
            "mod_type": "builtin mod",
            "mod_name": "distro",
            "mod_title": "Distro: "
        },
        {
            "mod_type": "builtin mod",
            "mod_name": "kernel",
            "mod_title": "Kernel: "
        },
        {
            "mod_type": "builtin mod",
            "mod_name": "device",
            "mod_title": "Device: "
        },
        {
            "mod_type": "builtin mod",
            "mod_name": "vendor",
            "mod_title": "Vendor: "
        },
        {
            "mod_type": "builtin mod",
            "mod_name": "ram",
            "mod_title": "Memory: "
        },
        {
            "mod_type": "env_var",
            "mod_name": "EDITOR",
            "mod_title": "Editor: "
        },
        {
            "mod_type": "builtin mod",
            "mod_name": "shell",
            "mod_title": "Shell: "
        },
        {
            "mod_type": "builtin mod",
            "mod_name": "cpu",
            "mod_title": "CPU: "
        },
        {
            "mod_type": "env_var",
            "mod_name": "XDG_CURRENT_DESKTOP",
            "mod_title": "DE: "
        }
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
