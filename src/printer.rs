use syntect::dumps;
use syntect::easy::HighlightLines;
use syntect::highlighting::Style;
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

pub fn printer(s: &str) {
    let ps = SyntaxSet::load_defaults_newlines();
    let th = dumps::from_binary(include_bytes!("../assets/Monokai.bin"));

    let syntax = ps.find_syntax_by_extension("json").unwrap();
    let mut h = HighlightLines::new(syntax, &th);
    for line in LinesWithEndings::from(s) {
        // LinesWithEndings enables use of newlines mode
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        println!("{}", escaped);
    }
}