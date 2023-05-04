#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Colour {
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
impl Colour {
    pub fn code(self) -> u8 {
        self as u8
    }
}
