use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

#[path = "./read_config/read_config.rs"]
mod read_config;

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
    //let path = Path::new("~/.config/codeopen/config.toml");

    //println!("{}", home_directory::get_home_dir());

    let config_str = read_config::get_config_file();

    let config: TOMLData = toml::from_str(&config_str).unwrap();

    let args: Vec<String> = env::args().collect();
    //println!("{}", args[1]);

    for i in &config.directory_shortcuts {
        if i.name == args[1].to_string() {
            let path = Path::new(&i.path);
            let newPath = path.to_str().unwrap();
            println!("{} {}", i.editor_alias, i.path);
            Command::new(&i.editor_alias)
                .args([&i.path])
                .spawn()
                .expect("Unable to open project");
            //let output = Command::new("whoami").output().expect("failed to do so");
        }
    }

    //println!("{:?}", config.directory_shortcuts);
}
