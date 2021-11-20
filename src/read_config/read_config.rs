mod home_directory;
use std::fs;
use std::path::Path;

pub fn get_config_file() -> String {
    let homedir = home_directory::get_home_dir();

    let configpath = format!("{}/.config/codeopen/config.toml", &homedir);

    let path = Path::new(&configpath);
    if !Path::new(&configpath).exists() {
        println!("Your config file at {} does not exist", &configpath);
        std::process::exit(1);
    }

    let config = fs::read_to_string(fs::canonicalize(&path).expect("Directory or file not found"))
        .expect("Config file does not exist");

    return config;
}
