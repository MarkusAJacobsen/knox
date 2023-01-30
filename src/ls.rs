use std::fs;
use std::fs::ReadDir;
use std::path::Path;
use crate::file_helpers::get_vault_path_string;

pub(crate) fn ls () {
    let dir_string = get_vault_path_string();
    let entries: ReadDir;

    if Path::exists(dir_string.as_ref()) {
        entries = fs::read_dir(dir_string).unwrap();
    } else {
        println!("Empty vault");
        return;
    }

    for entry in entries {
        let entry = entry.unwrap();
        println!("{}", entry.file_name().to_str().unwrap());
    }
}
