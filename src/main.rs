mod cli;
mod handlers;

use clap::Parser;
use cli::Cli;
use docs::Docs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let input = args.input;
    let command: cli::Command = args.command;
    let docs = Docs::from_file(input)?;

    match command {
        cli::Command::Generate { page } => handlers::generate(docs, page),
        cli::Command::Pages => handlers::get_pages(&docs)
            .iter()
            .for_each(|page| println!("{page}")),
        cli::Command::Types => handlers::get_types(docs),
    }

    Ok(())
}
