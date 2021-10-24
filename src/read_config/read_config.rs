mod home_directory;
use std::fs;
use std::path::Path;

pub fn get_config_file() -> String {
    let homedir = home_directory::get_home_dir();

    let configpath = format!("{}/.config/codeopen/config.toml", &homedir);

    let path = Path::new(&configpath);
    let config = fs::read_to_string(fs::canonicalize(&path).expect("Wrong path"))
        .expect("Config file does not exist");

    return config;
}
