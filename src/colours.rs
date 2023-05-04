use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(unused)]
pub enum C256 {
    SeaGreen = 108,
    MintyRose = 181,
    Plum = 183,
    SteelBlue = 67,
    RosyBrown = 138,
    Silver = 7,
    DarkMagenta = 91,
    Gray100 = 231,
    White = 15,
    SkyBlue1 = 117,
    Gray3 = 232,
    Gray50 = 244,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Basic {
    Red,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Colour {
    C256(C256),
    Basic(Basic),
}

impl From<Basic> for Colour {
    fn from(value: Basic) -> Self {
        Self::Basic(value)
    }
}
impl From<C256> for Colour {
    fn from(value: C256) -> Self {
        Self::C256(value)
    }
}
impl Display for Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Basic::Red => f.write_str("red"),
        }
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Colour::C256(v) => f.write_fmt(format_args!("{}", *v as u8)),
            Colour::Basic(b) => b.fmt(f),
        }
    }
}
