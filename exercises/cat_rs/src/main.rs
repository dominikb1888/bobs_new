use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::Parser;
use anyhow::Result;

use cat_rs::{Args, Config, Numbering, Case};


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
    // -U -L change text to upper or lowercase
    // More features can be added in lib.rs via enums of logical groups
    // There the Config struct translates between the individual flags and the config

    let config = Config::from(args);
    let mut line_counter = 0;
    for (line_num, line) in file.lines().enumerate() {
        let line_mod = match config.case {
            Case::Uppercase => line?.to_uppercase(),
            Case::Lowercase => line?.to_lowercase(),
            Case::None => line?,
        };

        let should_number = match config.numbering {
            Numbering::NonBlank => !line_mod.is_empty(), // Only if not empty
            Numbering::All => true,             // Always
            Numbering::None => false,           // Never
        };

        if should_number {
            line_counter += 1;
            println!("{:6}\t{}", line_counter, line_mod);
        } else {
            println!("{}", line_mod);
        }
    }

    Ok(())
}

