use std::{fs, io};
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

pub fn remove_dir_recursive(dir_path: &std::path::Path) -> io::Result<()> {
    if dir_path.is_dir() { 
        for entry in fs::read_dir(dir_path)? { 
            let entry = entry?;
            let entry_path = entry.path();
            
            if entry_path.is_dir() {
                remove_dir_recursive(&entry_path)?; 
            } else {
                fs::remove_file(entry_path)?; 
            }
        }
        fs::remove_dir(dir_path)?;
    }
    Ok(())
}

pub fn join_paths<T: AsRef<str>>(paths: &[T]) -> String {
    let joined = paths
        .iter()
        .map(|path| path.as_ref().trim_matches('/'))
        .collect::<Vec<_>>()
        .join("/");
    format!("/{}", joined)
}