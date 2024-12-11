use crate::Column::*;
use anyhow::{anyhow, bail, Result};
use clap::{ArgAction, Parser};
use std::{
    cmp::Ordering::*,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg()]
    file1: String,

    #[arg()]
    file2: String,

    /// Suppress printing of column 1
    #[arg(short('1'), action(ArgAction::SetFalse))]
    show_col1: bool,

    /// Suppress printing of column 2
    #[arg(short('2'), action(ArgAction::SetFalse))]
    show_col2: bool,

    /// Suppress printing of column 3
    #[arg(short('3'), action(ArgAction::SetFalse))]
    show_col3: bool,

    #[arg(short)]
    insensitive: bool,

    #[arg(short, long("output-delimiter"), default_value = "\t")]
    delimiter: String,
}

    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
}

// --------------------------------------------------
fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

// --------------------------------------------------
fn run(args: Args) -> Result<()> {
    ()
}

// --------------------------------------------------
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| anyhow!("{filename}: {e}"))?,
        ))),
    }
}
