pub mod color {
    pub static BLUE: u8 = 75;
    pub static GREEN: u8 = 150;
    pub static GREY: u8 = 245;
    pub static RED: u8 = 167;
    pub static YELLOW: u8 = 222;
}

pub mod symbol {
    pub const COMMAND: &str = "⬢";
    pub const INSERT: &str = "❯";

    pub const CONFLICT: &str = "✘";
    pub const DIRTY: &str = "●";
    pub const NEW: &str = "+";
    pub const NOCONFLICT: &str = "✔";
}
