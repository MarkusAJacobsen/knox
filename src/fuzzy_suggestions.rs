use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::path::Path;
use strsim::{jaro_winkler, levenshtein};
use crate::file_helpers::get_vault_path_string;

pub fn normalized_levenshtein_jaro_winkler(input: &str, candidate: &str) -> f64 {
    let jw_score = jaro_winkler(input, candidate);

    let max_len = input.len().max(candidate.len()) as f64;
    let lev_distance = levenshtein(input, candidate) as f64;
    let lev_score = if max_len == 0.0 { 1.0 } else { 1.0 - (lev_distance / max_len) };

    (jw_score * 0.6) + (lev_score * 0.4)
}

pub fn fuzzy_suggestions(input: &str, options: &[String]) -> Vec<String> {
    let mut scored: Vec<(&String, f64)> = options
        .iter()
        .map(|option| (option, normalized_levenshtein_jaro_winkler(input, option)))
        .collect();

    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
     
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