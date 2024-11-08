use std::env;
use std::fs;

fn main() {
   let args: Vec<String> = env::args().collect();
   //println!("{:?}", args);

    for arg in &args[1..] {  // Disregard first element
             // Open and read from a File
             let contents: String = fs::read_to_string(arg).expect("Something was wrong...");
             // Print file contents
             println!("{}", contents);
     }
}
