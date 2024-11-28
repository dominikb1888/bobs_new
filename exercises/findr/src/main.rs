use std::fs;
use walkdir::WalkDir;


fn main() {

    // I want to traverse filles and folders in rust?
    // Where are files folders?
    // Get data in a data structure about files and folders in current directory?
    let search_term = "main.rs";

    for entry_result in WalkDir::new("./") {
        match entry_result {
            Ok(entry) => {
                let entry_file_name = entry.file_name(); // Bind the file_name() result
                if let Some(name) = entry_file_name.to_str() {
                    if name == search_term {
                        println!("{:?}", entry.path()); // Print full path of the matched file
                    }
                }
            }
            Err(e) => eprintln!("Error reading entry: {}", e),
        }
    }
}
