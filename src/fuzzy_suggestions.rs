use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::path::Path;
use strsim::levenshtein;
use crate::file_helpers::get_vault_path_string;

pub fn fuzzy_suggestions(input: &str, options: &[String]) -> Vec<String> {
    let mut scored: Vec<(&String, usize)> = options
        .iter()
        .map(|option| (option, levenshtein(input, option)))
        .collect();

    scored.sort_by_key(|&(_, distance)| distance);

    scored.iter().take(3).map(|&(s, _)| s.clone()).collect()
}

pub fn get_fuzzy_suggestions(input: &str) -> Option<Vec<String>> {
    let dir_string = get_vault_path_string();
    let entries: ReadDir;

    if Path::exists(dir_string.as_ref()) {
        entries = fs::read_dir(dir_string).unwrap();
    } else {
        return None;
    }

    let entries_as_vector = entries
        .map(|res|res.unwrap())
        .collect::<Vec<DirEntry>>();

    let mut options: Vec<String> = Vec::new();
    for entry in entries_as_vector {
        let file_name = entry.file_name().into_string().unwrap();
        let path = Path::new(&file_name);

        options.push(path.with_extension("").to_str().unwrap().to_string());
    }
    
    if options.is_empty() {
        return None;
    }

    Some(fuzzy_suggestions(input, &options))
}