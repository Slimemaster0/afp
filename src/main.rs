// --- Use Section ---
mod logo;

// crate
use crate::logo::gen_logo;
// std
pub use std::process::Command; // Executing commands
use std::env; // Reading the environment
use std::fs::File; // Reading files
use std::io::prelude::*;

// colored
use colored::Colorize; // Colors

// serde
pub use serde::{Deserialize, Serialize};

// systemstat
use systemstat::{ System, Platform, saturating_sub_bytes }; // Hardware information
use systemstat::platform::PlatformImpl;
// --- End of Use Section ---

// --- Struct Section ---
// --- Exec ---
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
}
// --- End of Exec Struct ---



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

// --- End of Constants ---

fn main() { // main function
    // --- Modules ---
    // --- External Modules ---
    let osinfo = get_osinfo(); // get the OS information
    let user_name: String = get_user_name(); // get the user name
    let mut distro_logo = gen_logo(&osinfo);
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
    println!("{} {}", distro_logo.display(), str_user_host_name ); // Prints the user name and the hostname

    match env::var("XDG_SESSION_TYPE") { // Looks for the XDG_SESSION_TYPE EnvVar
        Ok(v) => println!("{} {} {}", distro_logo.display(), format!("Session Type:").blue().bold(), v), // Prints the XDG_SESSION_TYPE variable if it exits
        Err(_e) => nop() // Does nothing
    };

    println!("{} {}", distro_logo.display(), str_distro_name ); // Prints the distro name
    println!("{} {}", distro_logo.display(), str_kernel ); // Prints the kernel name and version
    println!("{} {}", distro_logo.display(), str_device ); // Prints the hardware model
    println!("{} {}", distro_logo.display(), str_vendor ); // Prints the hardware vendor
    println!("{} {}", distro_logo.display(), get_mem(&sys) ); // Prints the memory memory useage

    match env::var("EDITOR") { // Looks for the editor EnvVar
        Ok(v) => println!("{} {} {}", distro_logo.display(), format!("Editor:").blue().bold(), v), // Prints the EDITOR variable if it exits
        Err(_e) => nop() // Does nothing
    };

    match env::var("SHELL") { // Looks for the SHELL EnvVar
        Ok(v) => { 
            let shell: Vec<&str> = v.split("/").collect(); // Splits the string at '/'
            println!("{} {} {}", distro_logo.display(), format!("Shell:").blue().bold(), shell[shell.len() -1]); // Prints the SHELL variable if it exits

        },
        Err(_e) => nop() // Does nothing
    };

    println!("{} {}", distro_logo.display(), get_cpu_info(&sys)); // Prints the CPU information

    match env::var("XDG_CURRENT_DESKTOP") { // Looks for the XDG_CURRENT_DESKTOP EnvVar
        Ok(v) => println!("{} {} {}", distro_logo.display(), format!("DE:").blue().bold(), v), // Prints the XDG_CURRENT_DESKTOP variable if it exits
        Err(_e) => nop() // Does nothing
    };

    for _ in 0..distro_logo.remain { println!("{}", distro_logo.display()); } // Prints the rest of the distro logo
    // --- End of Print ---
} // --- End of main ---

// --- Common functions ---
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
// --- End of Convert Option<T> to normal types ---

fn nop() {} // A function that does nothing™
// --- End of common functions ---


// --- Out of main function Modules ---
fn get_osinfo() -> Osinfo { // Returns the OS information as 'Osinfo'
    let info_command = Exec { cmd: "hostnamectl".to_string(), args: vec!["--json=short".to_string()] }; // Returns the OS information as json 'String'

    let info_json_string = info_command.get_output(); // Runs 'info_command'
    let info_json_str: &str = &info_json_string; // Converts info_json_string to &str

    let osinfo: OsinfoOpt = serde_json::from_str(&info_json_str).expect("Err: could not parse json."); // Parses json 

    return osinfo.to_norm(); // Converts osinfo from OsinfoOpt to Osinfo and returns it
}

fn get_user_name() -> String {
    match env::var("USER") {
        Ok(v) => return v.to_string(),
        Err(_) => return {
            let command = Exec { cmd: "whoami".to_string(), args: vec![] };
            return command.get_output();
        }
    }
}

fn get_mem(sys: &PlatformImpl) -> String {
    match sys.memory() {
        Ok(mem) => {
            let mem_used = saturating_sub_bytes(mem.total, mem.free); // Used memory

            let free_frac: f64 = mem.total.as_u64() as f64 / mem_used.as_u64() as f64;

            if free_frac >= 2.7 {
                return format!("{} {}/{}", format!("Memory:").blue().bold(), format!("{}", mem_used).green(), mem.total);
            } else {
                if free_frac >= 1.3 {
                    return format!("{} {}/{}", format!("Memory:").blue().bold(), format!("{}", mem_used).yellow(), mem.total);
                } else {
                    return format!("{} {}/{}", format!("Memory:").blue().bold(), format!("{}", mem_used).red(), mem.total);
                }
            }
            
        },
        Err(_) => return "FUNCTION FAILURE".to_string(),
    }
}

// --- CPU information ---
fn get_cpu_info(sys: &PlatformImpl) -> String {
    let mut file = File::open("/proc/cpuinfo").expect("Err: cannot open /proc/cpuinfo");

    let mut cpuinfo = String::new();
    file.read_to_string(&mut cpuinfo).expect("Err: cant read /proc/cpuinfo");
    let cpu_vec: Vec<&str> = cpuinfo.split("\n").collect();

    let model_name: Vec<&str> = cpu_vec[4].split(": ").collect();
    let core_count: Vec<&str> = cpu_vec[12].split(": ").collect();
    let thread_count: Vec<&str> = cpu_vec[10].split(": ").collect();

    let cpu_temp = get_cpu_temp(&sys);

    if cpu_temp != "FUNCTION FAILURE".to_string() {
        return format!("{} {} ({}) [{}]", "CPU:".to_string().blue().bold(), model_name[1], core_count[1].green(), cpu_temp);
    } else { return format!("{} {} ({})", "CPU:".to_string().blue().bold(), model_name[1], core_count[1].green()); }
}

fn get_cpu_temp(sys: &PlatformImpl) -> String { // CPU temperature (As celcius)
    match sys.cpu_temp() {
        Ok(temp_float) => {
            let temp = temp_float as i32; // convert to int
            match temp {
                -300..=15 => return format!("{}{}", temp.to_string().blue(), "°C".to_string().blue() ), // From -300° to 15° (Cold)
                16..=29 => return format!("{}{}", temp.to_string().green(), "°C".to_string().green() ), // From 16° to 29° (Very good)
                30..=69 => return format!("{}{}", temp.to_string().yellow(), "°C".to_string().yellow() ), // from 30 to 69° (Normal)
                70..=10000 => return format!("{}{}", temp.to_string().red(), "°C".to_string().red() ), // 70°+ (HOT)
                _ => return format!("{}°C", temp.to_string())
            }
        },
        Err(_) => return "FUNCTION FAILURE".to_string(),
    }
}
