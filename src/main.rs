mod cli;
mod handlers;
mod models;

use clap::Parser;
use cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let input = args.input;
    let command: cli::Command = args.command;
    let docs = models::Docs::from_file(input)?;

    match command {
        cli::Command::Generate { pages } => handlers::generate(docs, pages),
        cli::Command::Pages => handlers::get_pages(&docs)
            .iter()
            .for_each(|page| println!("{page}")),
        cli::Command::Types => handlers::get_types(docs),
    }

    Ok(())
}
