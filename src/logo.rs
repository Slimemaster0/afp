// use
// Crate
use crate::Osinfo;
// Colored
use colored::Colorize;

// --- Structs ---
pub struct DistroLogo {
    logo_str: Vec<String>, // the logo it self
    empty_line: String, // What to show when the full logo has been printed out.
    pub remain: u8 // remaining lines
}

// --- DistroLogo impl ---
impl DistroLogo {
    pub fn display(&mut self) -> &String { // Display the current line of the logo.
        if self.remain != 0 { // If any lines remain return the current line.
            let line = self.logo_str.len() - usize::from(self.remain); // get the current line by subtracting the total number of lines by the remaining lines.
            self.remain -= 1; // Reduce remaining lines by one.
            return &self.logo_str[line]; // Return the current line.
        } else { return &self.empty_line; }
    }
}


pub fn gen_logo(os: &Osinfo) -> DistroLogo {
    let distro_name: &str = &os.OSPretty;
    match distro_name {
        "DEMO" => return demo(),
        
        _ => return DistroLogo { remain: 0, logo_str: vec![format!("{}", format!("").white())], empty_line: "".to_string() }
    }
}


// --- Distro Logos ---
fn demo() -> DistroLogo {
    let logo = DistroLogo { remain: 9,
        logo_str: vec![
        format!("{}", format!("--- 1 ---").blue().bold() ),
        format!("{}", format!("--- 2 ---").blue().bold() ),
        format!("{}", format!("--- 3 ---").blue().bold() ),
        format!("{}", format!("--- 4 ---").blue().bold() ),
        format!("{}", format!("--- 5 ---").blue().bold() ),
        format!("{}", format!("--- 6 ---").blue().bold() ),
        format!("{}", format!("--- 7 ---").blue().bold() ),
        format!("{}", format!("--- 8 ---").blue().bold() ),
        format!("{}", format!("--- 9 ---").blue().bold() ), ],
        empty_line: "         ".to_string() };
    return logo;
}
