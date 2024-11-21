use std::fs;
use walkdir::WalkDir;


fn main() {

    // I want to traverse filles and folders in rust?
    // Where are files folders?
    // Get data in a data structure about files and folders in current directory?

    for entry in WalkDir::new("./") {
        println!("{}", entry.unwrap().path().display());

    // TODO:
    // Compare our search with the files found in directory. Find out how to get a filename from an
    // entry in our WalkDir object

    // TODO:
    // What data type should we return? Add to a vector
    // Add a matching filepath if the filename is equal to this vector
    }

    // TODO:
    // Print Vector to STDOUT
}
