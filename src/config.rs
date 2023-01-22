// --- Use section ---
// Standard
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
// Crate
use crate::{ APP_NAME, CONFIG_FILE };
use crate::color::Kolor;
use crate::items::*;
// Serde
use serde::{ Serialize, Deserialize };
// --- End of use section ---


// --- Struct section ---
// config struct
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub color: Kolor,
    pub logo: String,
    pub items: Vec<Item>,
    pub allow_lazy: bool,
}
// --- End of struct section ---


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
                Err(_) => {

// --- Default config ---
                    let config_string: String = r#"
{
    "logo": "auto",
    "color": "None",
    "allow_lazy": true,
    "items": [
        {
            "UserHost": {
                "title": "",
                "color": "None"
            }
        },
        {
            "EnvVar": {
                "var": "XDG_SESSION_TYPE",
                "title": "Session Type: ",
                "color": "None"
            }
        },
        {
            "Distro": {
                "title": "Distro: ",
                "color": "None"
            }
        },
        {
            "Kernel": {
                "title": "Kernel: ",
                "color": "None"
            }
        },
        {
            "Device": {
                "title": "Device: ",
                "color": "None"
            }
        },
        {
            "Vendor": {
                "title": "Vendor: ",
                "color": "None"
            }
        },
        {
            "RAM": {
                "title": "Memory: ",
                "color": "None"
            }
        },
        {
            "EnvVar": {
                "var": "EDITOR",
                "title": "Editor: ",
                "color": "None"
            }
        },
        {
            "Shell": {
                "title": "Shell: ",
                "color": "None"
            }
        },
        {
            "GPU": {
                "title": "GPU$: ",
                "color": "None",
                "brand": true,
                "lazy": true
            }
        },
        {
            "CPU": {
                "title": "CPU: ",
                "color": "None"
            }
        },
        {
            "EnvVar": {
                "var": "XDG_CURRENT_DESKTOP",
                "title": "DE: ",
                "color": "None"
            }
        }
    ]
}
"#.to_string();
// --- End of Default config ---
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
