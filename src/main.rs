pub use std::process::Command; // Executing commands

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
#[allow(dead_code)]
struct Osinfo {
    codename: String,
    id: String,
    like: String,
    version: String,
}

fn main() {

let osinfo = get_osinfo();

println!("{} {} {}", format!("Distro:").blue().bold(), osinfo.id, format!("{}", osinfo.version).green() );



let user_name = Exec { cmd: "whoami".to_string(), args: vec![] };
let host_name = Exec { cmd: "uname".to_string(), args: vec!["-n".to_string()] };

println!("{}@{}", format!("{}", user_name.get_output()).blue().bold(), format!("{}", host_name.get_output()).green() );

let knl_name = Exec { cmd: "uname".to_string(), args: vec![] }; // Kernel name
let knl_ver = Exec { cmd: "uname".to_string(), args: vec!["-r".to_string()] }; // Kernel Version

println!("{} {} {}", format!("Kernel:").blue().bold(), knl_name.get_output(), format!("{}", knl_ver.get_output()).green() ); // Print output

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

fn get_osinfo() -> Osinfo {
    let info_command = Exec { cmd: "distro".to_string(), args: vec!["-j".to_string()] };

    let info_json_string = info_command.get_output();
    let info_json_str: &str = &info_json_string;

    let osinfo: Osinfo = serde_json::from_str(&info_json_str).expect("Err: could not parse json.");

    return osinfo;
}
