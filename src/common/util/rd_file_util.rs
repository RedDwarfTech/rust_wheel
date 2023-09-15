use std::{fs, io, path::PathBuf};
use log::error;
use std::path::Path;

pub fn create_folder_not_exists(folder: &String){
    match fs::create_dir_all(&folder) {
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
        fs::remove_dir_all(dir_path)?;
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

pub fn merge_paths(paths: &[&str]) -> String {
    let mut merged_path = PathBuf::new();
    for path in paths {
        merged_path.push(path);
    }
    merged_path.to_string_lossy().into_owned()
}

pub fn read_file(file_path: &String) -> Result<String, std::io::Error> {
    match fs::read_to_string(&file_path) {
        Ok(content) => {
            return Ok(content);
        }
        Err(error) => {
            return Err(error);
        }
    }
}