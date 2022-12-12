#![allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Color {
    Hex(String),
    Rgb(u32, u32, u32),
    Integer(u32),
}

impl Color {
    pub const WHITE: Color = Color::Rgb(255, 255, 255);
    pub const VERY_LIGHT_GRAY: Color = Color::Rgb(213, 212, 217);
    pub const LIGHT_GRAY: Color = Color::Rgb(157, 156, 161);
    pub const GRAY: Color = Color::Rgb(101, 101, 102);
    pub const DARK_GRAY: Color = Color::Rgb(73, 73, 74);
    pub const ALMOST_BLACK: Color = Color::Rgb(32, 32, 33);
    pub const BLACK: Color = Color::Rgb(1, 1, 1);

    pub const VERY_LIGHT_BLUE: Color = Color::Rgb(189, 198, 240);
    pub const LIGHT_BLUE: Color = Color::Rgb(168, 183, 230);
    pub const BLUE: Color = Color::Rgb(108, 139, 235);
    pub const DARK_BLUE: Color = Color::Rgb(67, 104, 217);
    pub const BLURPLE: Color = Color::Rgb(131, 118, 204);
    pub const VERY_DARK_BLUE: Color = Color::Rgb(34, 31, 166);

    pub const VERY_LIGHT_RED: Color = Color::Rgb(242, 162, 170);
    pub const LIGHT_RED: Color = Color::Rgb(230, 106, 118);
    pub const RED: Color = Color::Rgb(240, 41, 60);
    pub const DARK_RED: Color = Color::Rgb(148, 27, 38);
    pub const VERY_DARK_RED: Color = Color::Rgb(69, 5, 11);

    pub const VERY_LIGHT_YELLOW: Color = Color::Rgb(242, 209, 116);
    pub const LIGHT_YELLOW: Color = Color::Rgb(240, 209, 84);
    pub const YELLOW: Color = Color::Rgb(245, 207, 37);
    pub const DARK_YELLOW: Color = Color::Rgb(199, 165, 14);
    pub const VERY_DARK_YELLOW: Color = Color::Rgb(120, 98, 34);

    pub const LIGHT_BEIGE: Color = Color::Rgb(240, 200, 177);
    pub const BEIGE: Color = Color::Rgb(245, 194, 164);
    pub const LIGHT_ORANGE: Color = Color::Rgb(242, 147, 87);
    pub const ORANGE: Color = Color::Rgb(242, 113, 31);
    pub const DARK_ORANGE: Color = Color::Rgb(207, 84, 6);
    pub const BROWN: Color = Color::Rgb(145, 84, 45);
    pub const DARK_BROWN: Color = Color::Rgb(61, 31, 12);

    pub fn to_u32(&self) -> u32 {
        match self {
            Self::Hex(code) => u32::from_str_radix(code.replace('#', "").as_str(), 16).unwrap_or(0),
            Self::Rgb(r, g, b) => (r << 16u32) + (g << 8u32) + b,
            Self::Integer(int) => *int,
        }
    }
}
