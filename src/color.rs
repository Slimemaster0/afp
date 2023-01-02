use serde::{ Serialize, Deserialize };
use colored::{Color::*, Colorize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Kolor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightWhite,
    TrueColor(u8, u8, u8),
    None,
    Err
}

impl Kolor {
    pub fn to_ref(&self) -> &Kolor { return self; }
}

pub fn str_colorize(str: &String, c1: &Kolor, c2: &Kolor, c3: &Kolor) -> colored::ColoredString {
    match c3 {
        Kolor::Black => return str.black(),
        Kolor::Red => return str.red(),
        Kolor::Green => return str.green(),
        Kolor::Yellow => return str.yellow(),
        Kolor::Blue => return str.blue(),
        Kolor::Magenta => return str.magenta(),
        Kolor::Cyan => return str.cyan(),
        Kolor::White => return str.white(),
        Kolor::BrightBlack => return str.bright_black(),
        Kolor::BrightRed => return str.bright_red(),
        Kolor::BrightGreen => return str.bright_green(),
        Kolor::BrightYellow => return str.bright_yellow(),
        Kolor::BrightBlue => return str.bright_blue(),
        Kolor::BrightMagenta => return str.bright_magenta(),
        Kolor::BrightWhite => return str.bright_white(),
        Kolor::TrueColor(red, green, blue) => return str.truecolor(red.to_owned(), green.to_owned(), blue.to_owned()),
        Kolor::None => {
            let err = Kolor::Err;
            return str_colorize(str, &err, c1, c2);
        },
        Kolor::Err => panic!("Cant find color")
    }
}
