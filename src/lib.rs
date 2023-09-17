use std::{fs, io::Read, collections::HashMap, error::Error};
use sha2::{Sha512, Digest};
use walkdir::WalkDir;

pub fn folder_hash_sha512(folder_path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut hash_map: HashMap<String, String> = Default::default();

    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path().to_string_lossy().into_owned();
            let file_path_strip = file_path.replace(folder_path, "");

            let hash = file_hash_sha512(&file_path)?;

            hash_map.insert(file_path_strip, hash);
        }
    }

    Ok(hash_map)
}

pub fn file_hash_sha512(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = fs::File::open(file_path)?;
    let mut hasher = Sha512::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize().as_slice().to_owned();
    Ok(hex::encode(hash))
}