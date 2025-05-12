use std::path::Path;
use crate::file_helpers::get_vault_path_string;
use crate::fuzzy_suggestions::get_fuzzy_suggestions;

pub(crate) fn edit(name: &String) {
    let dir_name = & get_vault_path_string();

    let file_path_as_string = format!("{}/{}.txt", dir_name, name);

    if !Path::exists(file_path_as_string.as_ref()) {
        println!("Entry '{}' does not exist.", name);
        println!();
        println!("Create this by running 'knox new -n {}'", name);

        let fuzzy_suggestions = get_fuzzy_suggestions(name);
        match fuzzy_suggestions {
            Some(fuzzy_suggestions) => {
                println!("Or perhaps you meant one of these:");
                for suggestion in fuzzy_suggestions {
                    println!("\t{}", suggestion)
                }
                return
            }
            None => return
        }
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
