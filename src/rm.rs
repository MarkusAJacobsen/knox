use crate::file_helpers::get_vault_path_string;
use crate::fuzzy_suggestions::get_fuzzy_suggestions;

pub(crate) fn rm(name: &str) {
    let dir_name = get_vault_path_string();

    let file_path_as_string = dir_name + "/" + name + ".txt";

    let result = std::fs::remove_file(file_path_as_string);
    match result {
        Err(_err) => {
            println!("Entry '{}' does not exist.", name);

            let fuzzy_suggestions = get_fuzzy_suggestions(name);
            match fuzzy_suggestions {
                Some(fuzzy_suggestions) => {
                    println!("Or perhaps you meant one of these:");
                    for suggestion in fuzzy_suggestions {
                        println!("\t{}", suggestion)
                    }
                }
                None => return
            }
        }
        _ => {}
    }
}
