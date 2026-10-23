use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Eine Schach Engine")]
pub struct Args {
    #[arg(short, long)]
    pub perft: Option<u8>,
    #[arg(short = 'm', long)]
    pub generate_magics: bool,
}

pub fn cmdline() -> Args {
    Args::parse()
}
