// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use rasterizzatore::{
    Canvas,
    DumpToConsole, Rgba, Point,
};

fn main() {
    let mut canvas = Canvas::new(32, 32);

    canvas.clear(Rgba::BLUE);

    canvas.line(Point::new(1.0, 1.0), Point::new(2.0, 2.0), Rgba::RED);

    canvas.dump_to_console();
}
