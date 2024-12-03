use clap::{Parser, Subcommand};
use std::path::PathBuf;

use uuid::Uuid;

use stash_rs::stash::{Item, Stash};

use stash_rs::mem_stash::MemStash;

#[derive(Subcommand)]
enum Command {
    Add { name: String },
    Remove { uuid: Uuid },
    Inspect { uuid: Uuid },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let cli = Cli::parse();

    let mut s = MemStash::new();

    match cli.command {
        Command::Add { name } => {
            let uuid = s.add(Item::new(&name));
            println!("ADDED ENTRY: {uuid}");
        }
        Command::Remove { uuid } => s.remove(uuid),
        Command::Inspect { uuid } => s.inspect(uuid),
    };
}
