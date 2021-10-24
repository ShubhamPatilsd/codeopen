use std::env;
use std::fs;
use std::path::Path;
use toml::Value;

mod home_directory;

fn main() {
    //let path = Path::new("~/.config/codeopen/config.toml");

    println!("{}", home_directory::get_home_dir());

    format!();
    //println!("{:?}", fs::canonicalize(path));
    //let config = fs::read_to_string(fs::canonicalize(&path).expect("Wrong path"))
    //.expect("File does not exist");
    let args: Vec<String> = env::args().collect();
    println!("{}", args[1]);
    //println!("{}", config);
}
