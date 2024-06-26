use std::{env, fs, io};

// reads settings file from default project directory
pub fn read_settings(filename: &str) -> io::Result<String> {
    let current_dir = env::current_dir().expect("should be able to open directory");
    let path= current_dir.join(filename);
    println!("{:?}", path);
    return fs::read_to_string(path)
}

pub fn get_temp() -> String {
    let dir = env::temp_dir().into_os_string().into_string().unwrap();
    return dir
}