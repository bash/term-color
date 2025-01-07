//! This example shows how to retrieve the terminal's background color.

use terminal_colorsaurus::{background_color, Error, QueryOptions};

fn main() -> Result<(), display::DisplayAsDebug<Error>> {
    let bg = background_color(QueryOptions::default())?;
    let bg_8bit = bg.scale_to_8bit();
    println!("rgb16({}, {}, {})", bg.red, bg.green, bg.blue);
    println!("rgb8({}, {}, {})", bg_8bit.0, bg_8bit.1, bg_8bit.2);
    Ok(())
}

#[path = "../examples-utils/display.rs"]
mod display;
