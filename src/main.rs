pub use std::process::Command; // Executing commands
use std::env; // Reading the environment

use colored::Colorize; // Colors

pub use serde::{Deserialize, Serialize};

struct Exec {
    cmd: String,
    args: Vec<String>
}

impl Exec {

fn get_output(&self) -> String {
    let exec = &self.cmd;
    let mut cmd = Command::new(exec);

    // adding the arguments
    for arg in self.args.iter() {
        cmd.arg(arg);
    }

    match cmd.output() {
        Ok(o) =>  {
            unsafe {
                // Make a string from the output of the command.
                let mut str = String::from_utf8_unchecked(o.stdout);
                // Remove the new line charactor and return the output
                return trim_newline(&mut str);
            }
        },
        Err(e) => {
            println!("Err: {}", e);
            return "N/A".to_string();
        }
    }
}

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

fn main() {

let osinfo = get_osinfo();

println!("{} {}", format!("Distro:").blue().bold(), osinfo.OSPretty );



let user_name = Exec { cmd: "whoami".to_string(), args: vec![] };

println!("{}@{}", format!("{}", osinfo.Hostname).blue().bold(), format!("{}", user_name.get_output()).green() );

println!("{} {} {}", format!("Kernel:").blue().bold(), osinfo.KernelName, format!("{}", osinfo.KernelRelease).green() ); // Print output

println!("{} {}", format!("Device:").blue().bold(), osinfo.HardwareModel );

println!("{} {}", format!("Vendor:").blue().bold(), osinfo.HardwareVendor );

match env::var("EDITOR") {
    Ok(v) => println!("{} {}", format!("Editor:").blue().bold(), v),
    Err(_e) => nop()
};

}

fn trim_newline(s: &mut String) -> String {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    return s.to_string();
}

fn unopt_String(opt: Option<String>) -> String {
    match opt {
        Some(s) => return s,
        None => return "".to_string()
    }
}

impl OsinfoOpt {
    fn to_norm(self) -> Osinfo {
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

fn get_osinfo() -> Osinfo {
    let info_command = Exec { cmd: "hostnamectl".to_string(), args: vec!["--json=short".to_string()] };

    let info_json_string = info_command.get_output();
    let info_json_str: &str = &info_json_string;

    let osinfo: OsinfoOpt = serde_json::from_str(&info_json_str).expect("Err: could not parse json.");

    return osinfo.to_norm();
}

fn nop() {}
