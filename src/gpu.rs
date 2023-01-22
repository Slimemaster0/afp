use crate::{ APP_NAME, Item, Exec };
use std::fs::create_dir;
use std::{fs::File, path::PathBuf};
use std::io::{prelude, Write, Read};
use serde::__private::de::Content;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum GPUBrand {
    Intel,
    AMD,
    NVIDIA,
    Other
}

#[derive(Serialize, Deserialize)]
pub struct GPU {
    pub name: String,
    pub brand: String,
    pub short_brand: GPUBrand,
}

pub async fn get_gpu(items: &Vec<Item>, allow_lazy: &bool) -> Vec<GPU> {
    for item in items.iter() {
        match item {
            Item::GPU(gpu) => {
                if gpu.lazy && *allow_lazy {
                    match File::open(format!("/tmp/{}/gpu", APP_NAME).as_str()) {
                        Ok(f) => {
                            let mut file: File = f;
                            let mut lazy_file: String = String::new();
                            file.read_to_string(&mut lazy_file).expect("Cannot read file");
                            let gpu_vec: Vec<GPU> = serde_json::from_str(lazy_file.as_str())
                                .expect("Err: Could not parse /tmp/afp/gpu");
                            return gpu_vec;
                        },
                        Err(_) => return cold_start()
                    }
                } else {
                    return cold_start();
                }
            },
            _ => {}
        }
    }
    return Vec::new();
}

fn cold_start() -> Vec<GPU> {
    let lspci: Exec = Exec{ cmd: "lspci".to_string(), args: vec![String::from("-mm")] }; // Prepare the 'lspci' command
    let pci_table: String = lspci.get_output(); // Run the command and get the output
    let pci_table_vec: Vec<&str> = pci_table.split("\n").collect(); // Convert the command output to a 'Vector'
    
    let sep = "\"";

    let mut gpu_vec: Vec<GPU> = Vec::new();

    for item in pci_table_vec.iter() {
            if item.contains("VGA") | item.contains("3D") | item.contains("Display") {
                let gpu_str: Vec<&str> = item.split(sep).collect();
                let short_brand = get_short_brand(&gpu_str[3]);
                let gpu = GPU{ name: gpu_str[gpu_str.len() -2].to_string(), brand: gpu_str[3].to_string(), short_brand: short_brand };
                gpu_vec.push(gpu)
        }
    }
    crate_gpu_lazy(&gpu_vec);
    gpu_vec
}

fn get_short_brand(gpu_str: &&str) -> GPUBrand {
    match gpu_str {
        _ if gpu_str.contains("Advanced Micro Devices") => return GPUBrand::AMD,
        _ if gpu_str.contains("Intel") => return GPUBrand::Intel,
        _ if gpu_str.contains("NVIDIA") => return GPUBrand::NVIDIA,
        _ => return GPUBrand::Other
    }
}

fn crate_gpu_lazy(gpu_vec: &Vec<GPU>) {
    create_dir(format!("/tmp/{}", APP_NAME).as_str())
        .expect("Cannot create lazy directory"); // Creating the directory '/tmp/afp/' if it doesn't exist.
    let mut lazy_file = File::create(format!("/tmp/{}/gpu", APP_NAME).as_str())
        .expect("cant create file"); // Creating the file '/tmp/afp'

    let json_str = serde_json::to_string(gpu_vec)
        .expect("Cannot create json String"); // Creating the 'String' to write to 'lazy_file'
    lazy_file.write_all(json_str.as_bytes())
        .expect("cant crate file"); // Write 'json_str' to 'lazy_file'
}
