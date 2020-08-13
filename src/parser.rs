extern crate comrak;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use comrak::{parse_document, format_html, Arena, ComrakOptions};
use comrak::nodes::{AstNode, NodeValue};

pub fn parse_file(file_path: &str) -> Result<String, String>{
    
    let path = Path::new(file_path);
    let mut file = match File::open(path){
        Ok(file) => file,
        Err(_) => return Err(format!("Can not open file {}", path.file_name().unwrap().to_str().expect(""))),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents){
        Ok(_) => (),
        Err(_) => return Err(format!("Can not read file")),
    };

    let contents = contents.as_str();

    let arena = Arena::new();
    let parse_options = ComrakOptions::default();

    let root = parse_document(
        &arena,
        &contents,
        &parse_options);

    iterate_nodes(root, &|node| {
        match &mut node.data.borrow_mut().value {
            &mut NodeValue::Text(ref mut text) => {
                let orig = std::mem::replace(text, vec![]);
                *text = String::from_utf8(orig).unwrap().replace("my", "your").as_bytes().to_vec();
            }
            _ => (),
        }
    });

    let mut html = vec![];
    
    match format_html(root, &ComrakOptions::default(), &mut html){
        Ok(_) => (),
        Err(_) => return Err(format!("Could not parse file to html")),
    } ;
    
    format_html(root, &ComrakOptions::default(), &mut html).unwrap();

    Ok(String::from_utf8(html).unwrap())
}

fn iterate_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
    where F : Fn(&'a AstNode<'a>) {
    f(node);
    for c in node.children() {
        iterate_nodes(c, f);
    }
}
