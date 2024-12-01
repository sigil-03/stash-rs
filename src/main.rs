use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
enum Command {
    Add,
    Remove,
    Inspect,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

pub trait Stash {
    fn add(&self);
    fn remove(&self);
    fn inspect(&self);
}

struct TestStash {}

impl Stash for TestStash {
    fn add(&self) {
        todo!("add item to stash");
    }
    fn remove(&self) {
        todo!("remove item from stash");
    }
    fn inspect(&self) {
        todo!("inspect item in stash");
    }
}

fn main() {
    let cli = Cli::parse();

    let s = TestStash {};

    match cli.command {
        Command::Add => s.add(),
        Command::Remove => s.remove(),
        Command::Inspect => s.inspect(),
    };
}
