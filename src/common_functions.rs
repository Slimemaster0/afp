// --- Use Section ---
use crate::Osinfo;
use std::process::Command;
use std::path::PathBuf;
use std::env; // Reading the environment
use serde::Deserialize;

use colored::Colorize; // Colors

// --- Struct Section ---
// --- Exec ---
pub struct Exec { // A struct for running commands
    pub cmd: String, // The command
    pub args: Vec<String> // The arguments
}

impl Exec {
    pub fn get_output(&self) -> String { // Runs the command and returns the output.
        let mut cmd = Command::new(&self.cmd); // Converts the cmd string to a 'Command'

        // adding the arguments
        for arg in self.args.iter() {
            cmd.arg(arg);
        }

        match cmd.output() {
            Ok(o) =>  {
                unsafe {
                    let mut str = String::from_utf8_unchecked(o.stdout); // Make a string from the output of the command.
                    return trim_newline(&mut str); // Remove the new line charactor and return the output
                }
            },
            Err(e) => {
                println!("Err: {}", e);
                return "N/A".to_string();
            }
        }
    }
}
// --- End of Exec Struct ---


#[derive(Default, Clone, Deserialize)]
pub struct OsinfoOpt {
    HardwareVendor: Option<String>,
    HardwareModel: Option<String>,
    OperatingSystemPrettyName: Option<String>,
    FirmwareVersion: Option<String>,
    Hostname: Option<String>,
    KernelName: Option<String>,
    KernelRelease: Option<String>,
}

pub fn trim_newline(s: &mut String) -> String { // Trims new line chars
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    return s.to_string();

}

// Convert Option<T> to normal types
pub fn unopt_String(opt: Option<String>) -> String { // Converts 'Option<String>' to 'String'
    match opt {
        Some(s) => return s, // Return the String
        None => return "".to_string() // Return an empty string
    }
}

impl OsinfoOpt {
    pub fn to_norm(self) -> Osinfo { // Converts 'OsinfoOpt' to osinfo
        let opt = self;
        let hwv: String = unopt_String(opt.HardwareVendor);
        let hwm: String = unopt_String(opt.HardwareModel);
        let ospn: String = unopt_String(opt.OperatingSystemPrettyName);
        let fwv: String = unopt_String(opt.FirmwareVersion);
        let hn: String = unopt_String(opt.Hostname);
        let kn: String = unopt_String(opt.KernelName);
        let kr: String = unopt_String(opt.KernelRelease);

        let osi: Osinfo = Osinfo { HardwareVendor: hwv, HardwareModel: hwm, OSPretty: ospn, FirmwareVersion: fwv, Hostname: hn, KernelName: kn, KernelRelease: kr };

        return osi;
    }
}
// --- End of Convert Option<T> to normal types ---

pub fn nop() {} // A function that does nothing™

// --- User dirs ---
pub fn get_home_dir() -> PathBuf { // --- Home directory ---
    match env::var("HOME") { // 
        Ok(p) => { let path: PathBuf = PathBuf::from(p); return path; },
        Err(e) => panic!("{} {}", format!("Err:").red().bold(), e),
    }
}

pub fn get_config_dir() -> PathBuf { // --- Config directory ---
    match env::var("XDG_CONFIG_HOME") {
        Ok(c) => { let path: PathBuf = PathBuf::from(c); return path; },
        Err(_) => { 
            let mut configdir: PathBuf = get_home_dir();
            configdir.push(".config");
            return configdir;
        },
    }
}

pub fn get_osinfo() -> Osinfo { // Returns the OS information as 'Osinfo'
    let info_command = Exec { cmd: "hostnamectl".to_string(), args: vec!["--json=short".to_string()] }; // Returns the OS information as json 'String'

    let info_json_string = info_command.get_output(); // Runs 'info_command'
    let info_json_str: &str = &info_json_string; // Converts info_json_string to &str

    let osinfo: OsinfoOpt = serde_json::from_str(&info_json_str).expect("Err: could not parse json."); // Parses json 

    return osinfo.to_norm(); // Converts osinfo from OsinfoOpt to Osinfo and returns it
}
