use std::ffi::OsStr;
use std::path::Path;
use uuid::Uuid;

// build unique filename
pub fn uuid_file_name(file_path: &String) -> String {
    let uuid = Uuid::new_v4();
    let mut name = uuid.to_string();

    if let Some(ext) = Path::new(file_path).extension().and_then(OsStr::to_str) {
        name = format!("{}.{}", uuid, ext)
    }
    name
}

// copy file from /tmp to static/upload with new filename
pub fn save_file(path: &std::path::PathBuf, file_name: &String) {
    if let Err(error) = std::fs::copy(path, format!("static/upload/{}", file_name)) {
        println!("File error: {}", error);
    }
}

// delete file from static/upload
pub fn delete_file(file_name: &String) {
    if let Err(error) = std::fs::remove_file(format!("static/upload/{}", file_name)) {
        println!("File error: {}", error);
    }
}
