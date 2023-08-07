// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// This structure holds an RGBA color - red, green, blue, and alpha.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Rgba {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Rgba {

    /// Create a new RGBA color.
    #[inline]
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self { red, green, blue, alpha, }
    }

    /// Get the red component of this RGBA color.
    #[inline]
    pub const fn red(&self) -> u8 {
        self.red
    }

    /// Get the green component of this RGBA color.
    #[inline]
    pub const fn green(&self) -> u8 {
        self.green
    }

    /// Get the blue component of this RGBA color.
    #[inline]
    pub const fn blue(&self) -> u8 {
        self.blue
    }

    /// A completely white color `#FFFFFF`.
    pub const WHITE: Self = Self::new(255, 255, 255, 255);

    /// A completely black color `#FFFFFF`.
    pub const BLACK: Self = Self::new(0, 0, 0, 255);

    /// A completely red color `#FF0000`.
    pub const RED: Self = Self::new(255, 0, 0, 255);

    /// A completely green color `#00FF00`.
    pub const GREEN: Self = Self::new(0, 255, 0, 255);

    /// A completely blue color `#0000FF`.
    pub const BLUE: Self = Self::new(0, 0, 255, 255);

    /// A magenta color `#FF00FF`.
    pub const MAGENTA: Self = Self::new(255, 0, 255, 255);

    /// A cyan color `#00FFFF`.
    pub const CYAN: Self = Self::new(0, 255, 255, 255);

    /// A yellow color `#FFFF00`.
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
