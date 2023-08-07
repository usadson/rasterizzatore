// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{
    point::Point,
    rgba::Rgba,
};

/// The canvas is the thing on which can be drawn. This structure holds the
/// pixels for a certain image and provides functions to modify those values.
pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<Rgba>,
}

impl Canvas {

    /// Create a new canvas with a given width and height.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![Rgba::new(0, 0, 0, 0); width as usize * height as usize],
        }
    }

    /// Clear the whole canvas with a solid fill color.
    pub fn clear(&mut self, color: Rgba) {
        self.pixels.fill(color);
    }

    #[inline]
    fn set_pixel(&mut self, x: f32, y: f32, color: Rgba) {
        let x = x.round() as usize;
        let y = y.round() as usize;

        self.pixels[y * self.width as usize + x] = color;
    }

    /// Draw a line with a solid fill.
    pub fn draw_solid_line(&mut self, from: Point, to: Point, color: Rgba) {
        let dx = to.x() - from.x();
        let dy = to.y() - from.y();

        let step = if dx.abs() >= dy.abs() {
            dx.abs() as usize
        } else {
            dy.abs() as usize
        };

        let dx = dx / step as f32;
        let dy = dy / step as f32;

        let mut x = from.x();
        let mut y = from.y();
        let mut i = 0;

        while i <= step {
            self.set_pixel(x, y, color);
            x += dx;
            y += dy;
            i += 1;
        }
    }

    /// Get the width (in pixels) of the canvas.
    #[inline]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Get the height (in pixels) of the canvas.
    #[inline]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Get the color value of a given pixel.
    pub fn get_pixel(&self, x: u32, y: u32) -> Rgba {
        self.pixels[(y * self.width + x) as usize]
    }

    /// Get the raw pixel data.
    #[inline]
    pub fn raw_pixels(&self) -> &[Rgba] {
        &self.pixels
    }

}
