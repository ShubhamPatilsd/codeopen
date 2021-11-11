use serde::Deserialize;
use std::env;
use std::process::Command;

#[path = "./read_config/read_config.rs"]
mod read_config;

#[path = "./read_config/home_directory.rs"]
mod home_directory;

#[path = "./util/abs_path.rs"]
mod abs_path;

#[derive(Deserialize)]
struct TOMLData {
    directory_shortcuts: Vec<ShortcutMetaData>,
}

#[derive(Deserialize)]
struct ShortcutMetaData {
    name: String,
    path: String,
    editor_alias: String,
}

fn main() {
    let config_str = read_config::get_config_file();

    let config: TOMLData = toml::from_str(&config_str).unwrap();

    let args: Vec<String> = env::args().collect();

    for i in &config.directory_shortcuts {
        if i.name == args[1].to_string() {
            let absolute_string = abs_path::convert_path(&i.path);

            Command::new(&i.editor_alias)
                .arg(&absolute_string)
                .spawn()
                .expect("Unable to spawn editor process")
                .wait()
                .expect("ERR: Unable to open project in editor.");
        }
    }
}
