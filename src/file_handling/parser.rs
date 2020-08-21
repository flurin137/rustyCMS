extern crate comrak;

use comrak::{format_html, parse_document, Arena, ComrakOptions};

pub fn parse_file(data: &str) -> Result<String, String> {
    let arena = Arena::new();
    let parse_options = ComrakOptions::default();

    let root = parse_document(&arena, &data, &parse_options);

    let mut html = vec![];

    match format_html(root, &parse_options, &mut html) {
        Ok(_) => (),
        Err(_) => return Err(format!("Could not parse file to html")),
    };

    match String::from_utf8(html) {
        Ok(data) => Ok(data),
        Err(_) => Err(format!("Could not parse file to html")),
    }
}
