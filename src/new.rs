use crate::file_helpers::get_vault_path_string;

pub(crate) fn new(name: &String) {
    let dir_name = get_vault_path_string();

    let vim_command = "vim ".to_owned() + &*dir_name + "/" + name + ".txt";

    std::process::Command::new("/usr/bin/sh")
        .arg("-c")
        .arg(vim_command)
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");
}
