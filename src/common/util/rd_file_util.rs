use std::fs;
use log::error;

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