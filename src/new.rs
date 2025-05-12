use std::fs::File;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;
use crate::file_helpers::get_vault_path_string;

pub(crate) fn new(name: &String, generate: &bool) {
    let dir_name = & get_vault_path_string();

    let file_path_as_string = format!("{}/{}.txt", dir_name, name);

    if Path::exists(file_path_as_string.as_ref()) {
        println!("Entry '{}' already exists.", name);
        return;
    }

    if *generate == true {
        let _ok = match generate_uuid_secret_in_file(file_path_as_string) {
            Ok(_ok) => return,
            Err(error) => panic!("Could not generate new entry: {:?}", error),
        };
    }


    let vim_command = "vim ".to_owned() + dir_name + "/" + name + ".txt";

    std::process::Command::new("/bin/sh")
        .arg("-c")
        .arg(vim_command)
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");
}

fn generate_uuid_secret_in_file(file_path_as_string: String) -> std::io::Result<()> {
    let id = Uuid::new_v4();
    let mut file = File::create(file_path_as_string)?;
    write!(file, "{}", id)
}
