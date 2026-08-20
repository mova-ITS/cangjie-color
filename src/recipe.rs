// recipe.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Full,
    Letter(char),
    Unit(usize),
    GreyOnly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sign {
    pub letter: char,
    pub color: String,
    pub strokes: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipe {
    pub code: String,
    pub signs: Vec<Sign>,
}
