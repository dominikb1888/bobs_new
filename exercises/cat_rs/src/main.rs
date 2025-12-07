use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::Parser;
use anyhow::Result;

use cat_rs::Args;


fn main() {
    // starts the program
    // Print errors to STDERR
    // Exits the program
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    // gets data either from file or STDIN
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(args: Args) -> Result<()> { 
    // opens files
    // handles errors around file access
    for filename in &args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => {
               let _ = print_lines(file, &args);    
            },
        }
    }
    Ok(())
}

fn print_lines(file: Box<dyn BufRead>, args: &Args) -> Result<()> {
    // Iterates through each line and prints line based on arguments set
    // No argument: no line numbers
    // -b (number_nonblank_lines): Line numbers count does not include blank lines
    // -n (number_lines): Line numbers count includes blank lines
    let mut prev_num = 0;
    for (line_num, line) in file.lines().enumerate() {
        let line = line?;

        if args.number_lines {
            println!("{:>6}\t{line}", line_num + 1);
        } else if args.number_nonblank_lines {
            println!("{}", if line.is_empty() { "".into() } else { prev_num += 1;
format!("{prev_num:>6}\t{line}") });
        } else {
            println!("{line}");
        }
    }

    Ok(())
}

