extern crate find_folder;

use crate::file_handling::consts::*;
use find_folder::*;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub fn read_index() -> Result<String,String> {
    
    let user_path = match Search::ParentsThenKids(3, 3).for_folder(USER_FOLDER){
        Ok(path) => path,
        _ => return Err(format!("could not find folder {}", USER_FOLDER)),
    };

    let path_buffer = user_path.join(INDEX_FILE);

    let path : &Path = path_buffer.as_path();

    let file_name = match path.file_name(){
        Some(name) => name,
        None => return Err(format!("No valid file provided")), 
    };

    let file_name = match file_name.to_str(){
        Some(name) => name,
        None => return Err(format!("Unable to parse file name")), 
    };

    let mut file = match File::open(path){
        Ok(file) => file,
        Err(_) => return Err(format!("Can not open file {}", file_name)),
    };
    
    let mut file_contents = String::new();

    match file.read_to_string(&mut file_contents){
        Ok(_) => (),
        Err(_) => return Err(format!("Can not read file {}", file_name)),
    };

    Ok(file_contents)
}