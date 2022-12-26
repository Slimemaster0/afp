use std::path::PathBuf;
use crate::{ APP_NAME, CONFIG_FILE };
use std::fs::File;
use std::io::prelude::*;

// --- Configuration ---
// --- open config file ---
fn read_config_file(cd: &mut PathBuf) -> String {
    cd.push(APP_NAME);
    cd.push(CONFIG_FILE);

    let mut syscd = PathBuf::from("/etc");
    syscd.push(APP_NAME);
    syscd.push(CONFIG_FILE);
    

    match File::open(cd) {
        Ok(f) => {
            let mut file: File = f;
            let mut config_file: String = String::new();
            file.read_to_string(&mut config_file).expect("Err: could not read config.json");
            return config_file;
        },

        Err(_) => {
            match File::open(syscd) {
                Ok(f) => {
                    let mut file: File = f;
                    let mut config_file: String = String::new();
                    file.read_to_string(&mut config_file).expect("Err: could not read config.json");
                    return config_file;
                }
                Err(_) => { // Default config
                    let config_string: String = r#"
{
    
}
"#.to_string();
                    return config_string;
                }
            }
        }
    }
}

