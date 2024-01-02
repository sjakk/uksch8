#![allow(clippy::unnecessary_wraps)]

mod font;
mod processor;
mod display;

mod prelude {
pub const CHIP8_RAM:usize = 4096;
pub const CHIP8_WIDTH:usize = 64;
pub const CHIP8_HEIGHT:usize = 32;

pub use std::env;
pub use std::path::Path;
pub use crate::font::*;
pub use crate::processor::*;
pub use crate::display::*;

pub use ggez::{
    GameResult,Context,
    event,graphics,
    input::keyboard::{KeyCode,KeyInput},
};

}

use crate::prelude::*;


fn main() -> GameResult{

let args: Vec<_> = env::args().collect();

let (ctx, events_loop) = ggez::ContextBuilder::new("yahoou","breno <sjakk/ukase>")
    .window_setup(ggez::conf::WindowSetup::default().title("ayo"))
    .window_mode(ggez::conf::WindowMode::default().dimensions((64 * 20) as f32,(32 * 20) as f32))
    .build()?;


let mut state = Display::new();

let path = Path::new(&args[1]);

state.ch8.get_file(path)?;

event::run(ctx,events_loop,state)

}
