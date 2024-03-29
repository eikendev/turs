use ansi_term::Colour::Fixed;
use std::env;
use tico::tico;

use super::defaults;

fn shorten_path(cwd: &str) -> String {
    let path = match dirs::home_dir() {
        Some(path) => match path.to_str() {
            Some(path) => cwd.replace(path, "~"),
            _ => return String::from(""),
        },
        _ => return String::from(""),
    };

    tico(&path, Option::None)
}

pub fn display() {
    let path = match env::current_dir() {
        Ok(path) => shorten_path(path.to_str().unwrap()),
        _ => String::from("???"),
    };

    let path = Fixed(defaults::color::BLUE).paint(path);

    println!();
    println!("{path}");
}
