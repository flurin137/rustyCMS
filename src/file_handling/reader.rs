use crate::file_handling::consts::*;
use std::fs::{File};
use std::io::prelude::*;
use std::path::Path;

pub fn read_index(path: Option<String>) -> Result<String, String> {
   
    let sub_path = match path {
        Some(path) if path.starts_with("/") => format!("{}.md", &path[1..]),
        _ => String::from(INDEX_FILE),
    };
    
    println!("requested file: {}", sub_path);

    let sub_path = Path::new(&sub_path);
    
    let path: &Path = Path::new(USER_FOLDER);

    let path = &path.join(sub_path);

    let file_name = match path.file_name() {
        Some(name) => name,
        None => return Err(format!("No valid file provided")),
    };

    let file_name = match file_name.to_str() {
        Some(name) => name,
        None => return Err(format!("Unable to parse file name")),
    };

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err(format!("Can not open file {}", path.display())),
    };
    let mut file_contents = String::new();

    match file.read_to_string(&mut file_contents) {
        Ok(_) => (),
        Err(_) => return Err(format!("Can not read file {}", file_name)),
    };

    Ok(file_contents)
}
