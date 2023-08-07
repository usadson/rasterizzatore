// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

/// A point on the [Canvas][crate::Canvas], which is a 2D coordinate.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    /// Create a new point.
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Get the x coordinate of this point.
    #[inline]
    pub const fn x(&self) -> f32 {
        self.x
    }

    /// Get the y coordinate of this point.
    #[inline]
    pub const fn y(&self) -> f32 {
        self.y
    }
}
