use clap::{Parser, Subcommand};
use ratatui::{DefaultTerminal, Frame};
use std::path::PathBuf;

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();

    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: BibtlCommand,
}

#[derive(Subcommand)]
enum BibtlCommand {
    Dedup {
        #[arg(required = true, value_name = "FILE", num_args = 1..)]
        bib_paths: Vec<PathBuf>,
    },
    Review {
        #[arg(required = true, value_name = "FILE")]
        bib_path: PathBuf,
    },
}
