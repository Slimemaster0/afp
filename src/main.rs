// std
pub use std::process::Command; // Executing commands
use std::env; // Reading the environment

// Colord
use colored::Colorize; // Colors

// Serde
pub use serde::{Deserialize, Serialize};


struct Exec { // A struct for running commands
    cmd: String, // The command
    args: Vec<String> // The arguments
}

impl Exec {
    fn get_output(&self) -> String { // Runs the command and returns the output.
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
    // End of impl Exec
}



#[derive(Default, Clone, Deserialize)]
struct OsinfoOpt {
    HardwareVendor: Option<String>,
    HardwareModel: Option<String>,
    OperatingSystemPrettyName: Option<String>,
    FirmwareVersion: Option<String>,
    Hostname: Option<String>,
    KernelName: Option<String>,
    KernelRelease: Option<String>,
}

#[derive(Default, Clone, Deserialize)]
#[allow(dead_code)]
struct Osinfo {
    HardwareVendor: String,
    HardwareModel: String,
    OSPretty: String,
    FirmwareVersion: String,
    Hostname: String,
    KernelName: String,
    KernelRelease: String,
}

fn main() { // main function
    let osinfo = get_osinfo(); // get the OS information
    let user_name = Exec { cmd: "whoami".to_string(), args: vec![] }; // prepare the whoami command 

    println!("{} {}", format!("Distro:").blue().bold(), osinfo.OSPretty ); // Prints the distro name
    println!("{}@{}", format!("{}", osinfo.Hostname).blue().bold(), format!("{}", user_name.get_output()).green() ); // Prints the user name and the hostname
    println!("{} {} {}", format!("Kernel:").blue().bold(), osinfo.KernelName, format!("{}", osinfo.KernelRelease).green() ); // Prints the kernel name and version
    println!("{} {}", format!("Device:").blue().bold(), osinfo.HardwareModel ); // Prints the hardware model
    println!("{} {}", format!("Vendor:").blue().bold(), osinfo.HardwareVendor ); // Prints the hardware vendor

    match env::var("EDITOR") { // Looks for the editor EnvVar
        Ok(v) => println!("{} {}", format!("Editor:").blue().bold(), v), // Prints the EDITOR variable if it exits
        Err(_e) => nop() // Does nothing
    };
}

// Common functions

fn trim_newline(s: &mut String) -> String { // Trims new line chars
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    return s.to_string();
}

// Convert Option<T> to normal types

fn unopt_String(opt: Option<String>) -> String { // Converts 'Option<String>' to 'String'
    match opt {
        Some(s) => return s, // Return the String
        None => return "".to_string() // Return an empty string
    }
}

impl OsinfoOpt {
    fn to_norm(self) -> Osinfo { // Converts 'OsinfoOpt' to osinfo
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

fn get_osinfo() -> Osinfo { // Returns the OS information as 'Osinfo'
    let info_command = Exec { cmd: "hostnamectl".to_string(), args: vec!["--json=short".to_string()] }; // Returns the OS information as json 'String'

    let info_json_string = info_command.get_output(); // Runs 'info_command'
    let info_json_str: &str = &info_json_string; // Converts info_json_string to &str

    let osinfo: OsinfoOpt = serde_json::from_str(&info_json_str).expect("Err: could not parse json."); // Parses json 

    return osinfo.to_norm(); // Converts osinfo from OsinfoOpt to Osinfo and returns it
}

fn nop() {} // A function does nothing™
