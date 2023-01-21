use crate::color::Kolor;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone)]
pub enum Item {
    LineCount(LineCount),
    EnvVar(EnvVar),
    Command(Kommand),
    UserHost(UserHost),
    Distro(Distro),
    Kernel(Kernel),
    Device(Device),
    Vendor(Vendor),
    RAM(Memory),
    Shell(Shell),
    CPU(CPU),
    GPU(GPU),
}

// Line count
#[derive(Serialize, Deserialize, Clone)]
pub struct LineCount {
    pub command: String,
    pub args: Vec<String>,
    pub title: String,
    pub color: Kolor
}

// Environment Valueble
#[derive(Serialize, Deserialize, Clone)]
pub struct EnvVar {
    pub var: String,
    pub title: String,
    pub color: Kolor
}

// Command
#[derive(Serialize, Deserialize, Clone)]
pub struct Kommand {
    pub command: String,
    pub args: Vec<String>,
    pub title: String,
    pub color: Kolor
}

// UserHost
#[derive(Serialize, Deserialize, Clone)]
pub struct UserHost {
    pub title: String,
    pub color: Kolor
}

// Distro
#[derive(Serialize, Deserialize, Clone)]
pub struct Distro {
    pub title: String,
    pub color: Kolor
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Kernel {
    pub title: String,
    pub color: Kolor
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Device {
    pub title: String,
    pub color: Kolor
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Vendor {
    pub title: String,
    pub color: Kolor
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Memory {
    pub title: String,
    pub color: Kolor
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Shell {
    pub title: String,
    pub color: Kolor
}

#[derive(Serialize, Deserialize, Clone)]
pub struct  CPU {
    pub title: String,
    pub color: Kolor
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GPU {
    pub title: String,
    pub color: Kolor,
    pub brand: bool
}
