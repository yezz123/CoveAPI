use std::{fs::File, io::Read, path::Path};

pub fn read_file_to_string_or_err<E>(path: &Path, err: E) -> Result<String, E> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err(err),
    };

    let mut file_str = String::new();
    match file.read_to_string(&mut file_str) {
        Ok(_) => Ok(file_str),
        Err(_) => Err(err),
    }
}
