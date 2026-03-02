use bibtl::{Database, tui::App};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        BibtlCommand::Count { bib_paths } => {
            let mut total = 0;
            for path in bib_paths {
                match Database::from_bib(&path) {
                    Ok(db) => {
                        let len = db.len();
                        total += len;
                        println!("{}: {}", path.file_name().unwrap().display(), db.len());
                    }
                    Err(e) => {
                        println!("failed to parse database at path: {}", path.display());
                        if cli.verbose {
                            println!("-- {e}");
                        }
                    }
                }
            }

            println!("total: {total}");
        }
        BibtlCommand::Dedup { bib_paths } => todo!(),
        BibtlCommand::Review { bib_path } => {
            ratatui::run(|terminal| App::from_bib(bib_path).unwrap().run(terminal))?;
        }
    }

    Ok(())
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: BibtlCommand,

    #[arg(short)]
    verbose: bool,
}

#[derive(Subcommand)]
enum BibtlCommand {
    Count {
        #[arg(required = true, value_name = "FILE", num_args = 1..)]
        bib_paths: Vec<PathBuf>,
    },
    Dedup {
        #[arg(required = true, value_name = "FILE", num_args = 1..)]
        bib_paths: Vec<PathBuf>,
    },
    Review {
        #[arg(required = true, value_name = "FILE")]
        bib_path: PathBuf,
    },
}
