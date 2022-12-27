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
    let str_distro_name: String = format!("{} {}", format!("Distro:").blue().bold(), osinfo.OSPretty ); 
    let str_user_host_name: String = format!("{}@{}", format!("{}", user_name).green(), format!("{}", osinfo.Hostname).blue() );
    let str_kernel: String = format!("{} {} {}", format!("Kernel:").blue().bold(), osinfo.KernelName, format!("{}", osinfo.KernelRelease).green() );
    let str_device: String = format!("{} {}", format!("Device:").blue().bold(), osinfo.HardwareModel );
    let str_vendor: String = format!("{} {}", format!("Vendor:").blue().bold(), osinfo.HardwareVendor );

    // --- End of modules ---

    // --- Print ---


    for item_string in config.items.iter() {
        let item: &str = item_string;
        match item {
                "user,host" => println!("{} {}", distro_logo.display(), str_user_host_name ), // Prints the user name and the hostname

                "session type" => { match env::var("XDG_SESSION_TYPE") { // Looks for the XDG_SESSION_TYPE EnvVar
                    Ok(v) => println!("{} {} {}", distro_logo.display(), format!("Session Type:").blue().bold(), v), // Prints the XDG_SESSION_TYPE variable if it exits
                    Err(_e) => nop() // Does nothing
                }; 
            },

            "distro" => println!("{} {}", distro_logo.display(), str_distro_name ), // Prints the distro name
            "kernel" => println!("{} {}", distro_logo.display(), str_kernel ), // Prints the kernel name and version
            "device" => println!("{} {}", distro_logo.display(), str_device ), // Prints the hardware model
            "vendor" => println!("{} {}", distro_logo.display(), str_vendor ), // Prints the hardware vendor
            "ram" => println!("{} {}", distro_logo.display(), get_mem(&sys) ), // Prints the memory memory useage

            "editor" => {match env::var("EDITOR") { // Looks for the editor EnvVar
                    Ok(v) => println!("{} {} {}", distro_logo.display(), format!("Editor:").blue().bold(), v), // Prints the EDITOR variable if it exits
                    Err(_e) => nop() // Does nothing
                };
            },

            "shell" => { match env::var("SHELL") { // Looks for the SHELL EnvVar
                    Ok(v) => { 
                        let shell: Vec<&str> = v.split("/").collect(); // Splits the string at '/'
                        println!("{} {} {}", distro_logo.display(), format!("Shell:").blue().bold(), shell[shell.len() -1]); // Prints the SHELL variable if it exits

                    },
                    Err(_e) => nop() // Does nothing
                };
            },

            "cpu" => println!("{} {}", distro_logo.display(), get_cpu_info(&sys)), // Prints the CPU information

            "de" => { match env::var("XDG_CURRENT_DESKTOP") { // Looks for the XDG_CURRENT_DESKTOP EnvVar
                    Ok(v) => println!("{} {} {}", distro_logo.display(), format!("DE:").blue().bold(), v), // Prints the XDG_CURRENT_DESKTOP variable if it exits
                    Err(_e) => nop() // Does nothing
                };
            },

            _ => println!("{}", distro_logo.display())
        }
    }

    for _ in 0..distro_logo.remain { println!("{}", distro_logo.display()); } // Prints the rest of the distro logo

    // --- Debug Prints ---
    
    // println!("{}", configdir.to_str().expect("Err: can't print config directory"));

} // --- End of main ---
