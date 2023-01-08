use crate::color::{Kolor, self};
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
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
    CPU(CPU)
}

// Line count
#[derive(Serialize, Deserialize)]
pub struct LineCount {
    pub command: String,
    pub args: Vec<String>,
    pub title: String,
    pub color: Kolor
}

// Environment Valueble
#[derive(Serialize, Deserialize)]
pub struct EnvVar {
    pub var: String,
    pub title: String,
    pub color: Kolor
}

// Command
#[derive(Serialize, Deserialize)]
pub struct Kommand {
    pub command: String,
    pub args: Vec<String>,
    pub title: String,
    pub color: Kolor
}

// UserHost
#[derive(Serialize, Deserialize)]
pub struct UserHost {
    pub title: String,
    pub color: Kolor
}

// Distro
#[derive(Serialize, Deserialize)]
pub struct Distro {
    pub title: String,
    pub color: Kolor
}

#[derive(Serialize, Deserialize)]
pub struct Kernel {
    pub title: String,
    pub color: Kolor
}

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub title: String,
    pub color: Kolor
}

#[derive(Serialize, Deserialize)]
pub struct Vendor {
    pub title: String,
    pub color: Kolor
}

#[derive(Serialize, Deserialize)]
pub struct Memory {
    pub title: String,
    pub color: Kolor
}

#[derive(Serialize, Deserialize)]
pub struct Shell {
    pub title: String,
    pub color: Kolor
}

#[derive(Serialize, Deserialize)]
pub struct  CPU {
    pub title: String,
    pub color: Kolor
}
