use std::{io, sync::mpsc::Sender, thread};

use crate::{
    defs::FEN_START_POSITION,
    engine::EngineInput,
    types::error::ErrFatal,
    uci::{cmd::UciIn, Uci},
};

impl Uci {
    pub fn input_thread(&mut self, input_tx: Sender<EngineInput>) {
        let thread = thread::spawn(move || loop {
            let mut buffer = String::from("");
            io::stdin().read_line(&mut buffer).expect(ErrFatal::READ_IO);
            let cmd = Uci::parse_command(&buffer);
            let quit = cmd == UciIn::Quit;
            input_tx
                .send(EngineInput::Uci(cmd))
                .expect(ErrFatal::HANDLE);
            if quit {
                break;
            }
        });
        self.input_thread = Some(thread);
    }
}

impl Uci {
    fn parse_command(buffer: &str) -> UciIn {
        let input = buffer.trim_end().to_string();

        match input {
            cmd if cmd == "uci" => UciIn::Uci,
            cmd if cmd == "isready" => UciIn::IsReady,
            cmd if cmd == "ucinewgame" => UciIn::UciNewGame,
            cmd if cmd == "debug on" => UciIn::DebugOn,
            cmd if cmd == "debug off" => UciIn::DebugOff,
            cmd if cmd == "stop" => UciIn::Stop,
            cmd if cmd == "quit" => UciIn::Quit,
            cmd if cmd == "board" => UciIn::Board,
            cmd if cmd.starts_with("position") => position(&cmd),
            // cmd if cmd.starts_with("go") => go(&cmd),
            _ => UciIn::Unknown(input),
        }
    }
}

fn position(cmd: &str) -> UciIn {
    enum Tokens {
        Empty,
        Fen,
        Moves,
    }

    let stream: Vec<&str> = cmd.split_whitespace().collect();
    let mut fen = String::from("");
    let mut moves: Vec<String> = Vec::new();
    let mut skip_fen = false;
    let mut token = Tokens::Empty;

    for s in stream {
        match s {
            "position" => (),
            "startpos" => skip_fen = true,
            "moves" => token = Tokens::Moves,
            "fen" => {
                if !skip_fen {
                    token = Tokens::Fen;
                }
            }
            _ => match token {
                Tokens::Empty => (),
                Tokens::Fen => {
                    fen.push_str(s);
                    fen.push(' ');
                }
                Tokens::Moves => moves.push(String::from(s)),
            },
        }
    }

    if fen.is_empty() {
        fen = String::from(FEN_START_POSITION)
    }

    UciIn::Position(fen.trim().to_string(), moves)
}
