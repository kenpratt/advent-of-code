use std::{fs, path::Path};

pub fn read_file(filename: &str, relative_to: &str) -> String {
    let path = Path::new(relative_to).parent().unwrap().join(filename);
    fs::read_to_string(path).expect("Something went wrong reading the file")
}
