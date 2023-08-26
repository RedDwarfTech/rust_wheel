use std::fs;
use log::error;
use std::path::Path;

pub fn create_folder_not_exists(folder: &String){
    match fs::create_dir(&folder) {
        Ok(_) => {},
        Err(err) => {
            if err.kind() == std::io::ErrorKind::AlreadyExists {
                
            } else {
                error!("Failed to create folder: {},error: {}", folder, err);
            }
        }
    }
}

pub fn get_filename_without_ext(file_path: &String) -> &str{
    let path = Path::new(file_path);
    if let Some(file_name) = path.file_stem() {
        if let Some(file_name_str) = file_name.to_str() {
            return file_name_str;
        }
    }
    return "unkown";
}