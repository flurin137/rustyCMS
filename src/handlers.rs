use crate::file_handling::parser::*;
use crate::file_handling::reader::*;

pub fn handle_path(path: Option<String>) -> String {

    let file_content = match read_index(path){
        Ok(content) => content,
        Err(error) => return error,
    };

    match parse_file(file_content){
        Ok(html) => html,
        Err(error) => return error,
    }
}