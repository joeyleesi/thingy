use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Path to the exported_docs.json file
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,
}

#[derive(Subcommand)]
pub enum Command {
    /// Generate documentation for the given pages
    Generate {
        /// The pages to generate
        #[arg(short, long, value_delimiter = ',')]
        pages: Vec<String>,
    },
    /// List the possible pages to generate
    Pages,
    /// List the possible types used in the documentation (useful for creating link map)
    Types,
}
