use crate::engine::Engine;
#[cfg(feature = "wizardry")]
use crate::movegen::wizardry::generate_magics;

mod board;
mod defs;
mod engine;
mod evaluation;
mod misc;
mod movegen;
mod search;
mod types;
mod uci;

#[cfg(feature = "wizardry")]
fn main() {
    generate_magics();
}

#[cfg(not(feature = "wizardry"))]
fn main() {
    let mut engine = Engine::new();
    let result = engine.run();
    match result {
        Ok(()) => (),
        Err(error) => println!("{error}"),
    }
}
