#[derive(Clone, Debug, PartialEq, Hash, Default)]
pub struct DatabaseCursor {}

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct Entry {
    doi: String,
    title: String,
    abstr: String,
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
