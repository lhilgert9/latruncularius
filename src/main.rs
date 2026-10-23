use crate::engine::Engine;

mod board;
mod defs;
mod engine;
mod misc;
mod movegen;
mod types;
mod uci;

fn main() {
    let mut engine = Engine::new();
    let result = engine.run();
    match result {
        Ok(()) => (),
        Err(error) => println!("{error}"),
    }
}
