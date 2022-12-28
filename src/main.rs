// --- Use Section ---
mod logo;
mod common_functions;
mod cpu;
mod config;
mod smallmods;

// crate
use crate::common_functions::*;
use crate::smallmods::*;
use crate::logo::gen_logo;
use crate::cpu::*;
use crate::config::*;
use crate::Config;
// std
use std::path::PathBuf;
pub use std::process::Command; // Executing commands
use std::env; // Reading the environment

// colored
use colored::Colorize; // Colors

// serde
pub use serde::{Deserialize, Serialize};

// systemstat
use systemstat::{ System, Platform }; // Hardware information
// --- End of Use Section ---

// --- Struct Section ---


#[derive(Default, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Osinfo {
    HardwareVendor: String,
    HardwareModel: String,
    OSPretty: String,
    FirmwareVersion: String,
    Hostname: String,
    KernelName: String,
    KernelRelease: String,
}
// --- End of Struct section ---

// --- Constants ---
const APP_NAME: &str = "afp";
const CONFIG_FILE: &str = "config.json";
// --- End of Constants ---

fn main() { // main function
    let _homedir: PathBuf = get_home_dir(); // get the home directory
    let mut configdir: PathBuf = get_config_dir(); // get the config directory
    // --- Configuration ---
    let config: Config = read_config(&mut configdir);
     
    // --- Modules ---
    // --- External Modules ---
    let osinfo = get_osinfo(); // get the OS information
    let user_name: String = get_user_name(); // get the user name
    let mut distro_logo = gen_logo(&config.logo, &osinfo.OSPretty);
    let sys = System::new();

    // --- Long modules ---

    // --- Short Modules ---
    let str_distro_name: String = format!("{}", osinfo.OSPretty ); 
    let str_user_host_name: String = format!("{}@{}", format!("{}", user_name).green(), format!("{}", osinfo.Hostname).blue() );
    let str_kernel: String = format!("{} {}", osinfo.KernelName, format!("{}", osinfo.KernelRelease).green() );
    let str_device: String = format!("{}", osinfo.HardwareModel );
    let str_vendor: String = format!("{}", osinfo.HardwareVendor );

    // --- End of modules ---

    // --- Print ---


    for current_item in config.items.iter() {
        let item_type: &str = &current_item.mod_type;
        let item_name: &str = &current_item.mod_name;
        match item_type {
            "builtin mod" => {
                match item_name {
                    "user host" => println!("{}{}{}", distro_logo.display(), current_item.mod_title.blue().bold(), str_user_host_name ), // Prints the user name and the hostname

                    "distro" => println!("{}{}{}", distro_logo.display(), current_item.mod_title.blue().bold(), str_distro_name ), // Prints the distro name
                    "kernel" => println!("{}{}{}", distro_logo.display(), current_item.mod_title.blue().bold(), str_kernel ), // Prints the kernel name and version
                    "device" => println!("{}{}{}", distro_logo.display(), current_item.mod_title.blue().bold(), str_device ), // Prints the hardware model
                    "vendor" => println!("{}{}{}", distro_logo.display(), current_item.mod_title.blue().bold(), str_vendor ), // Prints the hardware vendor
                    "ram" => println!("{}{}{}", distro_logo.display(), current_item.mod_title.blue().bold(), get_mem(&sys) ), // Prints the memory memory useage

                    "shell" => { match env::var("SHELL") { // Looks for the SHELL EnvVar
                            Ok(v) => { 
                                let shell: Vec<&str> = v.split("/").collect(); // Splits the string at '/'
                                println!("{}{}{}", distro_logo.display(), current_item.mod_title.blue().bold(), shell[shell.len() -1]); // Prints the SHELL variable if it exits

                            },
                            Err(_e) => nop() // Does nothing
                        };
                    },

                    "cpu" => println!("{}{}{}", distro_logo.display(), current_item.mod_title.blue().bold(), get_cpu_info(&sys)), // Prints the CPU information


                    _ => println!("{}", distro_logo.display())
                }
            },
            "env_var" => {
                match env::var(item_name) {
                    Ok(v) => println!("{}{}{}", distro_logo.display(), current_item.mod_title.blue().bold(), v),
                    Err(_) => nop()
                }
            }
            _ => println!("{}", distro_logo.display())
        }
    }

    for _ in 0..distro_logo.remain { println!("{}", distro_logo.display()); } // Prints the rest of the distro logo

    // --- Debug Prints ---
    
    // println!("{}", configdir.to_str().expect("Err: can't print config directory"));

} // --- End of main ---
