use serde::Deserialize;
use std::env;
use std::fs;
use toml::Value;

#[path = "./read_config/read_config.rs"]
mod read_config;

#[derive(Deserialize)]
struct TOMLData {
    directory_shortcuts: Vec<ShortcutMetaData>,
    test: String,
}

#[derive(Deserialize)]
struct ShortcutMetaData {
    name: String,
    path: String,
    editor_alias: String,
}

fn main() {
    //let path = Path::new("~/.config/codeopen/config.toml");

    //println!("{}", home_directory::get_home_dir());

    let configStr = read_config::get_config_file();

    let config: TOMLData = toml::from_str(&configStr).unwrap();

    let args: Vec<String> = env::args().collect();
    println!("{}", args[1]);

    for i in &config.directory_shortcuts {
        println!("{:?}", i.name);
    }

    //println!("{:?}", config.directory_shortcuts);
}
