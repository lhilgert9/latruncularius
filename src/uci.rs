use std::{
    io,
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
};

use crate::{
    defs::About,
    engine::defs::{EngineOption, ErrFatal, Information, UiElement},
};

#[derive(PartialEq, Clone)]
pub enum UciReport {
    Uci,
    UciNewGame,
    IsReady,
    // SetOption(EngineOptionName),
    Position(String, Vec<String>),
    GoInfinite,
    GoDepth(i8),
    GoMoveTime(u128),
    GoNodes(usize),
    // GoGameTime(GameTime),
    Stop,
    Quit,

    Unknown,
}

pub enum UciControl {
    Quit,
    Identify,
    Ready,
    // SearchSummary(SearchSummary),
    // SearchCurrMove(SearchCurrentMove),
    // SearchStats(SearchStats),
    InfoString(String),
    // BestMove(Move),
}

pub struct Uci {
    report_handle: Option<JoinHandle<()>>,
    writing_handle: Option<JoinHandle<()>>,
    writing_tx: Option<Sender<UciControl>>,
}
impl Uci {
    pub fn new() -> Self {
        Self {
            report_handle: None,
            writing_handle: None,
            writing_tx: None,
        }
    }

    pub fn start(&mut self, report_tx: Sender<Information>, options: Arc<Vec<EngineOption>>) {
        self.report_thread(report_tx);
        self.control_thread(options);
    }

    pub fn send(&self, msg: UciControl) {
        if let Some(tx) = &self.writing_tx {
            tx.send(msg).expect(ErrFatal::CHANNEL);
        }
    }

    pub fn wait_for_shutdown(&mut self) {
        if let Some(h) = self.report_handle.take() {
            h.join().expect(ErrFatal::THREAD);
        }

        if let Some(h) = self.writing_handle.take() {
            h.join().expect(ErrFatal::THREAD);
        }
    }
}

impl Uci {
    fn report_thread(&mut self, report_tx: Sender<Information>) {
        let mut input = String::from("");

        let report_handle = thread::spawn(move || {
            let mut quit = false;
            while !quit {
                io::stdin().read_line(&mut input).expect(ErrFatal::READ_IO);
                let report = Uci::parse_input(&input);
                report_tx
                    .send(Information::Uci(report.clone()))
                    .expect(ErrFatal::HANDLE);
                quit = report == UciReport::Quit;
                input = String::from("");
            }
        });
        self.report_handle = Some(report_handle)
    }

    fn control_thread(&mut self, options: Arc<Vec<EngineOption>>) {
        let (writing_tx, writing_rx) = mpsc::channel::<UciControl>();
        let writing_handle = thread::spawn(move || {
            let mut quit = false;
            let engine_options = Arc::clone(&options);
            while !quit {
                let control = writing_rx.recv().expect(ErrFatal::CHANNEL);
                match control {
                    UciControl::Identify => {
                        Uci::id();
                        Uci::options(&engine_options);
                        Uci::uciok();
                    }
                    UciControl::Ready => Uci::readyok(),
                    UciControl::Quit => quit = true,
                    UciControl::InfoString(info) => Uci::info_string(&info),
                }
            }
        });
        self.writing_handle = Some(writing_handle);
        self.writing_tx = Some(writing_tx);
    }
}

impl Uci {
    fn parse_input(input: &str) -> UciReport {
        let i = input.trim_end().to_string();
        match i {
            cmd if cmd == "uci" => UciReport::Uci,
            cmd if cmd == "ucinewgame" => UciReport::UciNewGame,
            cmd if cmd == "isready" => UciReport::IsReady,
            cmd if cmd == "stop" => UciReport::Stop,
            cmd if cmd == "quit" || cmd == "exit" => UciReport::Quit,
            // cmd if cmd.starts_with("setoption") => Uci::parse_setoption(&cmd),
            // cmd if cmd.starts_with("position") => Uci::parse_position(&cmd),
            // cmd if cmd.starts_with("go") => Uci::parse_go(&cmd),
            _ => UciReport::Unknown,
        }
    }
}

impl Uci {
    fn id() {
        println!("id name {} {}", About::ENGINE, About::VERSION);
        println!("id author {}", About::AUTHOR);
    }

    fn options(options: &Arc<Vec<EngineOption>>) {
        for o in options.iter() {
            let name = format!("option name {}", o.name);

            let ui_element = match o.ui_element {
                UiElement::Spin => String::from("type spin"),
                UiElement::Button => String::from("type button"),
            };

            let value_default = if let Some(v) = &o.default {
                format!("default {}", (*v).clone())
            } else {
                String::from("")
            };

            let value_min = if let Some(v) = &o.min {
                format!("min {}", (*v).clone())
            } else {
                String::from("")
            };

            let value_max = if let Some(v) = &o.max {
                format!("max {}", (*v).clone())
            } else {
                String::from("")
            };

            let option = format!("{name} {ui_element} {value_default} {value_min} {value_max}")
                .trim()
                .to_string();

            println!("{option}");
        }
    }

    fn uciok() {
        println!("uciok");
    }

    fn readyok() {
        println!("readyok");
    }

    fn info_string(info: &str) {
        println!("info string {info}");
    }
}
