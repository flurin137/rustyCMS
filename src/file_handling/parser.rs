extern crate comrak;

use comrak::{parse_document, format_html, Arena, ComrakOptions};
use comrak::nodes::{AstNode, NodeValue};

pub fn parse_file(data: String) -> Result<String, String>{
    
    let arena = Arena::new();
    let parse_options = ComrakOptions::default();

    let root = parse_document(
        &arena,
        &data,
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
