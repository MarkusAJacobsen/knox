use crate::file_helpers::get_vault_path_string;
use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;

pub(crate) fn get(name: &str) {
    let dir_name = get_vault_path_string();

    let file_path_as_string = dir_name + "/" + name + ".txt";

    let result = std::fs::read_to_string(file_path_as_string);
    match result {
        Ok(mut contents) => {
            contents = String::from(contents.trim());

            let mut ctx = ClipboardContext::new().unwrap();
            let set_contents_result = ctx.set_contents(contents.into());
            match set_contents_result {
                Ok(_) => {
                    println!("Copied contents of {} to clipboard", name);
                }
                Err(err) => {
                    println!("Failed to copy contents of {} to clipboard: {}", name, err.to_string());
                }
            }
        },
        Err(_err) => {
            println!("Entry '{}' does not exist.", name);
            println!();
            println!("Create this by running 'knox new -n {}'", name)
        }
    }
}
