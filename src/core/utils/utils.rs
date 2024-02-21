use std::env;

pub fn get_os_newline() -> String {
    let os_type = env::consts::OS;

    let newline = match os_type {
        "windows" => "\r\n",
        _ => "\n"
    };
    return newline.to_string();
}