use std::{fs::File, io::Read, path::Path};

use biblatex::{Bibliography, ChunksExt};

#[derive(Clone, Debug, PartialEq, Hash, Default)]
pub struct Database {
    entries: Vec<Entry>,
}

impl Database {
    pub fn from_bib(bib_path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let mut file = File::open(bib_path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let bibliography = Bibliography::parse(&buf).unwrap();
        let entries: Vec<Entry> = bibliography
            .into_iter()
            .filter_map(|entry| {
                Some(Entry {
                    doi: entry.doi().ok()?,
                    title: entry.title().ok()?.format_verbatim(),
                    abstr: entry.abstract_().ok()?.format_verbatim(),
                })
            })
            .collect();

        Ok(Self { entries })
    }
}

impl Database {
    pub fn entry(&self, cursor: DatabaseCursor) -> &Entry {
        &self.entries[cursor.0]
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
    use biblatex::{Bibliography, ChunksExt};

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
}
