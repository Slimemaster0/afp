use std::env; // Reading the environment
use crate::Exec;
use colored::Colorize; // Colors

// systemstat
use systemstat::{ Platform, saturating_sub_bytes }; // Hardware information
use systemstat::platform::PlatformImpl;

pub fn get_user_name() -> String {
    match env::var("USER") {
        Ok(v) => return v.to_string(),
        Err(_) => return {
            let command = Exec { cmd: "whoami".to_string(), args: vec![] };
            return command.get_output();
        }
    }
}

pub fn get_mem(sys: &PlatformImpl) -> String {
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
