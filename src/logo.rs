// use
// Colored
use colored::Colorize;
use crate::color::Kolor;
use crate::color::Kolor::*;
use crate::color::str_colorize;
use crate::string_ext::StringExt;

// --- Structs ---
pub struct DistroLogo {
    logo_str: Vec<String>, // the logo it self
    empty_line: String, // What to show when the full logo has been printed out.
    pub remain: u8, // remaining lines
    pub color: Kolor
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


pub fn gen_logo(auto: &String, os: &String, color: &Kolor) -> DistroLogo {
    let auto_str: &str = auto;

    match auto_str {
        "DEMO" => return demo(),
        "Arch Linux" => return archlinux(color),
        
        "auto" => return gen_logo(&os, &os, color),
        
        _ => return DistroLogo { remain: 0, logo_str: vec![format!("{}", format!("").white())], empty_line: "".to_string(), color: Blue }

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
        empty_line: "         ".to_string(),
        color: Blue };
    return logo;
}

fn archlinux(color: &Kolor) -> DistroLogo {
    let logo = DistroLogo { remain: 19,
        logo_str: vec! [
            format!("{}",  str_colorize(format!("                c                ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!("               ;M:               ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!("              .MMM'              ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!("              cMMMW.             ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!("                .MMW             ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!("                  KMN            ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!("           OOkd;   MMX           ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!("          XMMMMMWc MMM0          ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!("         KMMMMMMMMNMMMMO         ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!("        0MMMMMMMMMMMMMMMx        ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!("       OMMMMMMMMMMMMMMMMMl       ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!("      xMMMMMMMMMMW0d:            ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!("     dMMMMMMMMMMMMMMMWKx:.       ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!("    cMMMMMMMMMMMMMMMMMMMMMKc     ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!("   ,MMMMMMMMMMx     .MMMMMMMW.   ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!("  .MMMMMMMMMMW       :MMMMMMMW.  ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!(" .WMMMMMMMMMX         dMMMMMMMN  ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!(".WMMMo                      .MMN ").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
            format!("{}",  str_colorize(format!("X                               ;").to_ref(), Kolor::Err.to_ref(), Kolor::Blue.to_ref(), color).bold()),
        ],
        empty_line: "                                       ".to_string(),
        color: Blue };
    return logo;
}

