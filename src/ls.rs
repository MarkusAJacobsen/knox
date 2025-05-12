use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::path::Path;
use crate::file_helpers::get_vault_path_string;
use ansi_term::Colour::{Yellow};

pub(crate) fn ls () {
    println!("{}", Yellow.underline().paint("Stored entries:"));

    let dir_string = get_vault_path_string();
    let entries: ReadDir;

    if Path::exists(dir_string.as_ref()) {
        entries = fs::read_dir(dir_string).unwrap();
    } else {
        println!("{:ident$}Empty vault", "", ident=2);
        return;
    }
    
    let mut entries_as_vector = entries
        .map(|res|res.unwrap())
        .collect::<Vec<DirEntry>>();
    
    entries_as_vector.sort_by_key(|dir|dir.file_name());
    for entry in entries_as_vector {
        let file_name = entry.file_name().into_string().unwrap();
        let path = Path::new(&file_name);
        println!("{:ident$}{}", "", path.with_extension("").to_str().unwrap(), ident=2);
    }
}
