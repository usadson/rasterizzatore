// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Rgba {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Rgba {

    #[inline]
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self { red, green, blue, alpha, }
    }

    #[inline]
    pub const fn red(&self) -> u8 {
        self.red
    }

    #[inline]
    pub const fn green(&self) -> u8 {
        self.green
    }

    #[inline]
    pub const fn blue(&self) -> u8 {
        self.blue
    }

    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    pub const BLACK: Self = Self::new(0, 0, 0, 255);

    pub const RED: Self = Self::new(255, 0, 0, 255);
    pub const GREEN: Self = Self::new(0, 255, 0, 255);
    pub const BLUE: Self = Self::new(0, 0, 255, 255);

    pub const MAGENTA: Self = Self::new(255, 0, 255, 255);
    pub const CYAN: Self = Self::new(0, 255, 255, 255);
    pub const YELLOW: Self = Self::new(255, 255, 0, 255);
}

#[cfg(feature = "dump-colored")]
impl From<Rgba> for colored::Color {
    fn from(value: Rgba) -> Self {
        Self::TrueColor {
            r: value.red(),
            g: value.green,
            b: value.blue()
        }
    }
}

#[cfg(feature = "dump-colored")]
impl From<Rgba> for colored::CustomColor {
    fn from(value: Rgba) -> Self {
        Self::new(value.red(), value.green(), value.blue())
    }
}
