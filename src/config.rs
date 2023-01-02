use std::path::PathBuf;
use crate::{ APP_NAME, CONFIG_FILE };
use std::fs::File;
use std::io::prelude::*;
use serde::{ Serialize, Deserialize };
use crate::color::Kolor;

// printmodule struct
#[derive(Serialize, Deserialize)]
pub struct PrintMod {
    pub module: String, // The name of the module, the name of the builtin module or environment variable.
    pub title: String,
    pub args: Vec<String>,
    pub color: Kolor
}

// config struct
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub color: Kolor,
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
    "color": "None",
    "items": [
        {
            "module": "user host",
            "args": [],
            "title": "",
            "color": "None"
        },
        {
            "module": "env_var",
            "args": [ "XDG_SESSION_TYPE" ],
            "title": "Session Type: ",
            "color": "None"
        },
        {
            "module": "distro",
            "args": [],
            "title": "Distro: ",
            "color": "None"
        },
        {
            "module": "kernel",
            "args": [],
            "title": "Kernel: ",
            "color": "None"
        },
        {
            "module": "device",
            "args": [],
            "title": "Device: ",
            "color": "None"
        },
        {
            "module": "vendor",
            "args": [],
            "title": "Vendor: ",
            "color": "None"
        },
        {
            "module": "ram",
            "args": [],
            "title": "Memory: ",
            "color": "None"
        },
        {
            "module": "env_var",
            "args": [ "EDITOR" ],
            "title": "Editor: ",
            "color": "None"
        },
        {
            "module": "shell",
            "args": [],
            "title": "Shell: ",
            "color": "None"
        },
        {
            "module": "cpu",
            "args": [],
            "title": "CPU: ",
            "color": "None"
        },
        {
            "module": "env_var",
            "args": [ "XDG_CURRENT_DESKTOP" ],
            "title": "DE: ",
            "color": "None"
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
