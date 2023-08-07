// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use crate::{
    Canvas,
    Rgba,
};

/// Some symbols which are useful for dumping pixels to the console.
mod symbols {
    pub const FULL_BLOCK: char = '█';
    pub const DARK_SHADE: char = '▓';
    pub const MEDIUM_SHADE: char = '▒';
    pub const LIGHT_SHADE: char = '░';
}

/// This trait extends the types of this crate with a utility function
/// `dump_to_console`.
///
/// Dumping to the console can be useful, e.g. for debugging the draw calls
/// without setting up a whole graphics stack, which can be hard or slow to do.
///
/// Deriving from [`Debug`] or [`Display`][core::fmt::Display] can otherwise
/// lead to accidental dumping of large data structures. Another advantage is
/// that this functionality can be disabled with the appropriate feature flags.
pub trait DumpToConsole {

    /// Dump this type to the console.
    fn dump_to_console(&self);

}

#[cfg(feature = "dump-colored")]
fn dump_rgba_colored(avg: usize, color: Rgba) {
    if !colored::control::SHOULD_COLORIZE.should_colorize() {
        dump_rgba_normal(avg);
        return;
    }

    use self::symbols::FULL_BLOCK;
    use colored::Colorize;
    print!("{}", format!("{FULL_BLOCK}{FULL_BLOCK}").custom_color(color.into()));
}

fn dump_rgba_normal(avg: usize) {
    use symbols::*;
    let char = if avg < 52 {
        ' '
    } else if avg < 104 {
        LIGHT_SHADE
    } else if avg < 156 {
        MEDIUM_SHADE
    } else if avg < 210 {
        DARK_SHADE
    } else {
        FULL_BLOCK
    };

    print!("{char}{char}");
}

impl DumpToConsole for Rgba {
    fn dump_to_console(&self) {
        let total = self.red() as usize
            + self.green() as usize
            + self.blue() as usize;
        let avg = total / 3;


        #[cfg(feature = "dump-colored")]
        {
            dump_rgba_colored(avg, *self);
        }

        #[cfg(not(feature = "dump-colored"))]
        {
            dump_rgba_normal(avg)
        }
    }
}

impl DumpToConsole for Canvas {
    fn dump_to_console(&self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.get_pixel(x, y).dump_to_console();
            }
            println!();
        }
    }
}
