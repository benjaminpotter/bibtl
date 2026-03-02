use biblatex::{Bibliography, Chunk, Chunks, ChunksExt, ParseError, Spanned};
use std::{fs::File, io::Read, path::Path};
use thiserror::Error;

pub mod tui;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("failed to parse biblatex: {0}")]
    BibLatexError(#[from] ParseError),

    #[error("failed to open database")]
    IOError(#[from] std::io::Error),
}

#[derive(Clone, Debug, PartialEq, Hash, Default)]
pub struct Database {
    entries: Vec<Entry>,
}

impl Database {
    pub fn from_bib(bib_path: impl AsRef<Path>) -> Result<Self, DatabaseError> {
        let mut file = File::open(bib_path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let bibliography = Bibliography::parse(&buf)?;
        let entries: Vec<Entry> = bibliography
            .into_iter()
            .filter_map(|entry| {
                Some(Entry {
                    doi: entry.doi().ok()?,
                    title: entry.title().ok()?.format_verbatim(),
                    abstr: entry
                        .abstract_()
                        .ok()
                        .map_or(String::new(), ChunksExt::format_verbatim),
                })
            })
            .collect();

        Ok(Self { entries })
    }

    #[must_use]
    pub fn from_entries(entries: impl IntoIterator<Item = Entry>) -> Self {
        Self {
            entries: entries.into_iter().collect(),
        }
    }

    #[must_use]
    pub fn to_bib(&self) -> String {
        let mut bibliography = Bibliography::new();

        for (i, entry) in self.entries.iter().enumerate() {
            let key = format!("ref{i:05}");
            let entry_type = biblatex::EntryType::Article;
            let mut bl_entry = biblatex::Entry::new(key, entry_type);

            let title = [Spanned::detached(Chunk::Verbatim(entry.title.clone()))].to_vec();
            let abstr = [Spanned::detached(Chunk::Verbatim(entry.abstr.clone()))].to_vec();

            bl_entry.set_doi(entry.doi.clone());
            bl_entry.set_title(title);
            bl_entry.set_abstract_(abstr);

            bibliography.insert(bl_entry);
        }

        bibliography.to_biblatex_string()
    }

    #[must_use]
    pub fn entry(&self, cursor: DatabaseCursor) -> &Entry {
        &self.entries[cursor.0]
    }

    #[must_use]
    pub fn entries(&self) -> Vec<&Entry> {
        self.entries.iter().collect()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Default)]
pub struct DatabaseCursor(usize);

impl DatabaseCursor {
    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Default)]
pub struct Entry {
    pub doi: String,
    pub title: String,
    pub abstr: String,
}

#[cfg(test)]
mod tests {
    use biblatex::{Bibliography, ChunksExt, ParseError, ParseErrorKind, Token};

    #[test]
    fn load_bibtex() {
        let src = "@book{tolkien1937, author = {J. R. R. Tolkien}, title = {The Lord of the Rings}, abstract = {An adventure!}}";
        let bibliography = Bibliography::parse(src).unwrap();
        let entry = bibliography.get("tolkien1937").unwrap();
        let author = entry.author().unwrap();
        let title = entry.title().unwrap().format_verbatim();
        let abstr = entry.abstract_().unwrap().format_verbatim();
        assert_eq!(author[0].name, "Tolkien");
        assert_eq!(title, "The Lord of the Rings");
        assert_eq!(abstr, "An adventure!");
    }

    #[test]
    fn load_bibtex_eof() {
        const SRC: &str = include_str!("../tests/fixtures/eof.bib");
        assert_eq!(
            Bibliography::parse(SRC),
            // symbols $ and % in the abstract cause eof
            Err(ParseError {
                span: 1928..1928,
                kind: biblatex::ParseErrorKind::UnexpectedEof,
            })
        );
    }

    #[test]
    fn load_text() {
        const SRC: &str = include_str!("../tests/fixtures/wos.txt");
    }
}
