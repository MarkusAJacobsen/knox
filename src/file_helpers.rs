pub fn get_vault_path_string() -> String {
    let documents = dirs::document_dir().unwrap();
    let documents_path_as_string = String::from(documents.to_string_lossy());
    let create_vault_path: String = documents_path_as_string + "/.cargo";
    return create_vault_path;
}
