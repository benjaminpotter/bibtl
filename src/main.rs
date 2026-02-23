use bibtl::{Database, DatabaseCursor};
use clap::{Parser, Subcommand};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};
use std::path::{Path, PathBuf};

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        BibtlCommand::Dedup { bib_paths } => todo!(),
        BibtlCommand::Review { bib_path } => {
            ratatui::run(|terminal| App::from_bib(bib_path)?.run(terminal))?;
        }
    }

    Ok(())
}

#[derive(Debug, Default)]
struct App {
    database: Database,
    cursor: DatabaseCursor,
    state: AppState,
}

impl App {
    fn from_bib(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        Ok(Self {
            database: Database::from_bib(path)?,
            ..Default::default()
        })
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while self.state != AppState::Done {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if crossterm::event::read()?.is_key_press() {
            self.state = AppState::Done;
        }

        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let entry = self.database.entry(self.cursor).clone();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(10), Constraint::Fill(1)])
            .split(area);

        Paragraph::new(entry.title)
            .block(
                Block::new()
                    .title("Publication Title")
                    .borders(Borders::ALL),
            )
            .render(layout[0], buf);

        Paragraph::new(entry.abstr)
            .wrap(Wrap::default())
            .block(
                Block::new()
                    .title("Publication Abstract")
                    .borders(Borders::ALL),
            )
            .render(layout[1], buf);
    }
}

#[derive(Debug, Default, PartialEq)]
enum AppState {
    #[default]
    Running,
    Done,
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
