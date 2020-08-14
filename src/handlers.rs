use crate::file_handling::parser::*;
use crate::file_handling::reader::*;

pub fn index() -> String {

    let file_content = match read_index(){
        Ok(content) => content,
        Err(error) => return error,
    };

    match parse_file(file_content){
        Ok(html) => html,
        Err(error) => return error,
    }
}