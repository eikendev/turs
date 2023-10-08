use super::defaults;

const COMMAND_KEYMAP: &str = "vicmd";
const NO_ERROR: &str = "0";

pub fn display(keymap: &str, last_return_code: &str) {
    let symbol = match keymap {
        COMMAND_KEYMAP => defaults::symbol::COMMAND,
        _ => defaults::symbol::INSERT,
    };

    let color = match (symbol, last_return_code) {
        (defaults::symbol::COMMAND, _) => defaults::color::YELLOW,
        (_, NO_ERROR) => defaults::color::GREEN,
        _ => defaults::color::RED,
    };

    print!("%F{{{color}}}{symbol}%f ");
}
