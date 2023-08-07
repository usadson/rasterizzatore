// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn x(&self) -> f32 {
        self.x
    }

    #[inline]
    pub const fn y(&self) -> f32 {
        self.y
    }
}
