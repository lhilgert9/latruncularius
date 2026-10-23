use std::{sync::mpsc::channel, thread};

use crate::{
    types::error::ErrFatal,
    uci::{cmd::UciOut, Uci},
};

impl Uci {
    pub fn output_thread(&mut self) {
        let (output_tx, output_rx) = channel::<UciOut>();
        let thread = thread::spawn(move || loop {
            let cmd = output_rx.recv().expect(ErrFatal::CHANNEL);
            match cmd {
                UciOut::Id => {
                    println!(
                        "id name {} {}",
                        env!("CARGO_PKG_NAME"),
                        env!("CARGO_PKG_VERSION")
                    );
                    println!("id author {}", env!("CARGO_PKG_AUTHORS"));
                    println!("uciok");
                }
                UciOut::ReadyOk => println!("readyok"),
                UciOut::Quit => break,
                UciOut::InfoString(message) => println!("info string {message}"),
                UciOut::Custom(message) => println!("{message}"),
            }
        });
        self.output_thread = Some(thread);
        self.output_tx = Some(output_tx)
    }
}
