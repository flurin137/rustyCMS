use crate::file_handling::parser::*;
use crate::file_handling::reader::*;

pub fn handle_path(path: Option<&str>) -> String {
    let file_content = match read_markdown_file(path){
        Ok(content) => content,
        Err(error) => return error,
    };

    match parse_file(&file_content){
        Ok(html) => html,
        Err(error) => error,
    }
}

pub fn handle_static_path(file_name: &str) -> String {

    match read_style_sheet(file_name){
        Ok(file_content) => file_content,
        Err(error) => error,
    }
}