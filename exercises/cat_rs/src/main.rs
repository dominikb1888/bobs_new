use std::fs;

fn main() {
   let filename = "test.txt";
   let file: String = fs::read_to_string(filename).expect("File does not exist");
   println!("{}", file);
}
