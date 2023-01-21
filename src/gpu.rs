use crate::Exec;

pub enum GPUBrand {
    Intel,
    AMD,
    NVIDIA,
    Other
}

pub struct GPU {
    pub name: String,
    pub brand: String,
    pub short_brand: GPUBrand,
}

pub async fn get_gpu() -> Vec<GPU> {
    let lspci: Exec = Exec{ cmd: "lspci".to_string(), args: vec![String::from("-mm")] };
    let pci_table: String = lspci.get_output();
    let pci_table_vec: Vec<&str> = pci_table.split("\n").collect();
    
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
