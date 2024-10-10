fn main() {
    // Read in Data (Text ideally)
    // Check if data is Text
    // Print the data [X]
    let clargs: Vec<String> = std::env::args().collect();
    for arg in clargs {
        print!("{} ", arg);
    }
}

