#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NonPrompt {
    DarkMagenta = 91,
    Gray100 = 231,
    White = 15,
    SkyBlue1 = 117,
    Gray3 = 232,
    Gray50 = 244,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prompt {
    SeaGreen = 108,
    MintyRose = 181,
    Plum = 183,
    SteelBlue = 67,
    RosyBrown = 138,
    Silver = 7,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Colour {
    NonPrompt(NonPrompt),
    Prompt(Prompt),
}
impl Colour {
    pub fn code(self) -> u8 {
        match self {
            Colour::NonPrompt(p) => p as u8,
            Colour::Prompt(p) => p as u8,
        }
    }
}
impl From<NonPrompt> for Colour {
    fn from(value: NonPrompt) -> Self {
        Self::NonPrompt(value)
    }
}
impl From<Prompt> for Colour {
    fn from(value: Prompt) -> Self {
        Self::Prompt(value)
    }
}
