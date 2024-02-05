use std::{env, fs, io};

pub fn read_settings(filename: &str) -> io::Result<String> {
    let current_dir = env::current_dir().expect("should be able to open directory");
    let path= current_dir.join(filename);
    return fs::read_to_string(path)
}