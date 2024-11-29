
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ops::Range;
use anyhow::{anyhow, bail, Result, Error};

#[derive(Debug, Parser)]
struct Args {
     /// Input files
    #[arg()]
    file: String,

       /// Specify a delimiter for field splitting
    #[arg(short, long, default_value = "\t", value_name = "DELIMITER", value_parser = validate_delimiter_length)]
    delimiter: String,

    #[command(flatten)]
    extract: ArgsExtract,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
struct ArgsExtract {
    /// Specify a range of bytes to cut
    #[arg(short, long, value_parser = parse_range, value_name = "BYTES")]
    bytes: Option<Range<usize>>,

    /// Specify a range of characters to cut
    #[arg(short, long, value_parser = parse_range, value_name = "CHARS", conflicts_with("bytes"))]
    chars: Option<Range<usize>>,

    /// Specify a range of fields to cut
    #[arg(short, long, value_parser = parse_range, value_name = "FIELDS",  conflicts_with("chars"))]
    fields: Option<Range<usize>>,
}



fn validate_delimiter_length(val: &str) -> Result<String, Error> {
    // Check if the string has a length of 1
    if val.len() == 1 {
        Ok(val.to_string()) // Return the valid string as-is
    } else {
        bail!(r#"--delim "{}" must be a single byte"#, val.to_string());
    }
}

/// Parse a string range like "1-5" into a `Range<usize>`
fn parse_range(arg: &str) -> Result<Range<usize>, String> {
    if arg.contains("-") {
        let parts: Vec<&str> = arg.split('-').collect();
        if parts.len() != 2 {
            return Err("Range must be in the format start-end".to_string());
        }

        let start = parts[0].parse::<usize>().map_err(|_| "Invalid start range")?;
        let end = parts[1].parse::<usize>().map_err(|_| "Invalid end range")?;

        Ok(start-1..end)
    } else {
        let pos: usize = arg.parse().unwrap();
        Ok(pos-1..pos)
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Storage for processed lines
    let mut lines: Vec<Vec<String>> = Vec::new();

    // Process each input file
    let file = args.file;
        // Open the file, default to stdin if "-"
        let input: Box<dyn BufRead> = if file == "-" {
            Box::new(io::BufReader::new(io::stdin()))
        } else {
            Box::new(io::BufReader::new(File::open(Path::new(&file)).expect("Failed to open file")))
        };

        // Read and process each line
        for line in input.lines() {
            let line = line.expect("Failed to read line");

            if let Some(range) = &args.extract.fields {
                // Split by the delimiter and keep the selected fields
                let fields: Vec<String> = line.split(&args.delimiter.to_string()).map(String::from).collect();
                lines.push(fields[range.clone()].to_vec());
            } else if let Some(range) = &args.extract.chars {
                // Take only the selected character range
                let chars: String = line.chars().collect::<Vec<_>>()[range.clone()].iter().collect();
                lines.push(vec![chars]);
            } else if let Some(range) = &args.extract.bytes {
                // Take only the selected byte range
                let bytes = &line.as_bytes()[range.clone()];
                lines.push(vec![String::from_utf8_lossy(bytes).to_string()]);
            } else {
                // Default to pushing the whole line if no ranges are provided
                lines.push(vec![line]);
            }
        }


    // Output the processed lines
    for line in lines {
        println!("{}", line.join(&args.delimiter.to_string()));
    }

    Ok(())
}

