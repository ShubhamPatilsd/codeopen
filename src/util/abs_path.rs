#[path = "../read_config/home_directory.rs"]
mod home_directory;

pub fn convert_path(path: &str) -> String {
    let mut absolute_string = path.clone().to_string();
    let new_string: String;

    if absolute_string.chars().next() == Some('~') {
        absolute_string.remove(0);
        new_string = home_directory::get_home_dir() + &absolute_string;
    } else if absolute_string.chars().next() == Some('/') {
        new_string = absolute_string.clone();
    } else {
        new_string = home_directory::get_home_dir() + "/" + path;
    }

    return new_string;
}
