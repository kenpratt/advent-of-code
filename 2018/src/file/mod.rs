use std::{fs, path::Path};

pub const INPUT_FILE: &'static str = "input.txt";
pub const EXAMPLE_FILE: &'static str = "example.txt";

pub fn read_file_(filename: &str, relative_to: &str) -> String {
    let path = Path::new(relative_to).parent().unwrap().join(filename);
    fs::read_to_string(path).expect("Something went wrong reading the file")
}

#[allow(unused_macros)]
macro_rules! read_file {
    ($l:expr) => {
        read_file_($l, file!())
    };
}
#[allow(unused_imports)]
pub(crate) use read_file;

macro_rules! read_input_file {
    () => {
        read_file_(INPUT_FILE, file!())
    };
}
pub(crate) use read_input_file;

#[allow(unused_macros)]
macro_rules! read_example_file {
    () => {
        read_file_(EXAMPLE_FILE, file!())
    };
}
#[allow(unused_imports)]
pub(crate) use read_example_file;
