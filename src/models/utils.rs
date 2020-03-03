// copy file from /tmp to static/upload with new filename
pub fn copy_file(path: &std::path::PathBuf, file_name: String) {
    if let Err(error) = std::fs::copy(path, format!("static/upload/{}", file_name)) {
        println!("File error: {}", error);
    }
}

// delete file from static/upload
pub fn delete_file(file_name: String) {
    if let Err(error) = std::fs::remove_file(format!("static/upload/{}", file_name)) {
        println!("File error: {}", error);
    }
}
