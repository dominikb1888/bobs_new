use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
struct Args {
    #[arg(
        value_name = "FILE",
        default_value = "-"
    )]
    files: Vec<String>,
    #[clap(short, long)]
    lines: bool,
    #[clap(short, long)]
    words: bool,
    #[clap(short('c'), long)]
    bytes: bool,
    #[clap(short('m'), long, conflicts_with("bytes"))]
    chars: bool,
}

#[derive(Debug, PartialEq)]
struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

fn main() {
    let args = Args::parse();
    run(args).expect("ERROR")
}

fn run(mut args: Args) -> Result<()> {
    if [args.lines, args.words, args.bytes, args.chars].iter().all(|v| !v) {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    for filename in &args.files {
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(file) => {
                let info = count(file)?;
                println!("{:?}", info);
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }

        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}
