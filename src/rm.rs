use crate::file_helpers::get_vault_path_string;

pub(crate) fn rm(name: &str) {
    let dir_name = get_vault_path_string();

    let file_path_as_string = dir_name + "/" + name + ".txt";

    let result = std::fs::remove_file(file_path_as_string);
    match result {
        Err(err) => {
            println!("File {} not found. Err: {}", name, err.to_string());
        }
        _ => {}
    }
}
