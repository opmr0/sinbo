use clap::{Parser, Subcommand};

mod storage;
mod commands;

use storage::Storage;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    Get {
        name: String,
        #[arg(short, long)]
        copy: bool,
    },
    Add {
        name: String,
        #[arg(long, short, num_args = 1)]
        file_name: String,
        #[arg(short, long, num_args = 1..)]
        tags: Option<Vec<String>>,
    },
    List {
        #[arg(short, long, num_args = 1..)]
        tags: Option<Vec<String>>,
    },
    Remove {
        name: String,
    },
    Edit {
        name: String,
    },
}


fn main() {
    let args = Cli::parse();
    let storage = Storage::new();

    match args.action{
        Action::Get { name, copy } => todo!(),
        Action::Add { name, file_name, tags } => todo!(),
        Action::List { tags } => todo!(),
        Action::Remove { name } => todo!(),
        Action::Edit { name } => todo!(),
    }
}
