pub fn get_home_dir() -> String {
    let homedirectory: String;

    match home::home_dir() {
        Some(path) => {
            homedirectory = path
                .into_os_string()
                .into_string()
                .expect("oops cant find homedirectory")
        }
        None => {
            println!("ERROR: cannot get home directory");
            homedirectory = String::from("UNABLE TO FIND HOME DIRECTORY");
        }
    }

    return homedirectory;
}
