

use crate::Exec;

use colored::Colorize; // Colors

use std::fs::File; // Reading files
use systemstat::{ System, Platform, saturating_sub_bytes }; // Hardware information
use systemstat::platform::PlatformImpl;
use std::io::prelude::*;

// --- CPU information ---
pub fn get_cpu_info(sys: &PlatformImpl) -> String {
    let mut file = File::open("/proc/cpuinfo").expect("Err: cannot open /proc/cpuinfo"); // Open /proc/cpuinfo

    let mut cpuinfo = String::new(); // Create a to to store the file contents
    file.read_to_string(&mut cpuinfo).expect("Err: cant read /proc/cpuinfo"); // Write the contents of /proc/cpuinfo to the 'cpuinfo' String
    let cpu_vec: Vec<&str> = cpuinfo.split("\n").collect(); // Turn line in the cpuinfo String into a variable in a 'vector'

    let model_name: Vec<&str> = cpu_vec[4].split(": ").collect(); // Model Name
    let core_count: Vec<&str> = cpu_vec[12].split(": ").collect(); // Core count
    let _thread_count: Vec<&str> = cpu_vec[10].split(": ").collect(); // Thread count

    let cpu_temp = get_cpu_temp(&sys); // Get the CPU temperature

    if cpu_temp != "FUNCTION FAILURE".to_string() { // Returning the CPU model, core count and temperature ( If afp was able to the temperature )
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
// --- End of CPU information ---


