use std::{fs::File, io::Read, path::Path};

use crate::models::EndpointConfiguration;

use super::print_debug_message;

pub fn read_file_to_string_or_err<E>(path: &Path, err: E) -> Result<String, E> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(why) => {
            print_debug_message(why.to_string());
            return Err(err);
        }
    };

    let mut file_str = String::new();
    match file.read_to_string(&mut file_str) {
        Ok(_) => Ok(file_str),
        Err(_) => Err(err),
    }
}

pub fn print_endpoints<'a, T: Iterator<Item = &'a EndpointConfiguration>>(endpoints: T) {
    for endpoint in endpoints {
        println!(
            "- \"{}\", {:?}, {}",
            endpoint.path, endpoint.method, endpoint.status_code
        );
    }
}
