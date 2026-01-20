use std::borrow::Cow;

struct NameStats<'a> {
    text: Cow<'a, str>,
    char_count: usize,
}

impl<'a> NameStats<'a> {
    fn new<S>(input: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        let text: Cow<'a, str> = input.into();

        Self {
            char_count: text.len(),
            text, // no extra clone needed
        }
    }

    fn display(&self) {
        println!(
            "Stored text: \"{}\" | Total characters counted: {}",
            self.text, self.char_count
        );
    }
}

fn main() {
    let stats = NameStats::new("John");

    stats.display();
}
