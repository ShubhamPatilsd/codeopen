use serde::Deserialize;
use std::process::Command;

use structopt::StructOpt;

#[path = "./read_config/read_config.rs"]
mod read_config;

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

#[derive(StructOpt)]
struct Cli {
    // The name to look for
    #[structopt(help = "Alias defined for a project in your config.toml file")]
    name: String,
}

fn main() {
    let config_str = read_config::get_config_file();

    let config: TOMLData = toml::from_str(&config_str).unwrap();

    let args = Cli::from_args();

    for i in &config.directory_shortcuts {
        if i.name == args.name.to_string() {
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
