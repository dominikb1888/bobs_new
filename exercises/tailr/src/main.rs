
use clap::Parser;
use anyhow::{anyhow, Result, bail};
use std::fmt::Error;
use std::fs::File;

#[derive(Debug, Parser)]
struct Args {
    #[arg(required = true)]
    files: Vec<String>,

    #[arg(value_name = "LINES", short('n'), long, default_value = "10")]
    lines: String,

    #[arg(value_name = "BYTES", short('c'), long, conflicts_with("lines"))]
    bytes: Option<String>,

    #[arg(short, long)]
    quiet: bool,
}

#[derive(Debug, PartialEq)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

use crate::TakeValue::*;

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let lines = parse_num(args.lines)
        .map_err(|e| anyhow!("illegal line count -- {e}"))?;

    let bytes = args
        .bytes
        .map(parse_num)
        .transpose()
        .map_err(|e| anyhow!("illegal byte count -- {e}"))?;

    for filename in args.files {
        match File::open(&filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(_) => {
                let (total_lines, total_bytes) =
                count_lines_bytes(&filename)?;
                println!(
                    "{filename} has {total_lines} lines, {total_bytes} bytes"
                );
            }
        }
    }
    Ok(())
}

fn parse_num(val: String) -> Result<TakeValue> {
    unimplemented!()
}

fn count_lines_bytes(filename: &str) -> Result<(i64, i64)> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::{parse_num, count_lines_bytes, TakeValue::*};

    #[test]
    fn test_parse_num_negative_integer() {
        let res = parse_num("3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));
    }

    #[test]
    fn test_parse_num_positive_integer() {
        let res = parse_num("+3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(3));
    }

    #[test]
    fn test_parse_num_explicit_negative() {
        let res = parse_num("-3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));
    }

    #[test]
    fn test_parse_num_zero() {
        let res = parse_num("0".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(0));
    }

    #[test]
    fn test_parse_num_plus_zero() {
        let res = parse_num("+0".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), PlusZero);
    }

    #[test]
    fn test_parse_num_max_boundaries() {
        let res = parse_num(i64::MAX.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));
    }

    #[test]
    fn test_parse_num_min_boundaries() {
        let res = parse_num(i64::MIN.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN));
    }

    #[test]
    fn test_parse_num_invalid_float() {
        let res = parse_num("3.14".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "3.14");
    }

    #[test]
    fn test_parse_num_invalid_string() {
        let res = parse_num("foo".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo");
    }

    #[test]
    fn test_count_lines_one() {
        let res = count_lines_bytes("tests/inputs/one.txt");
        assert!(res.is_ok());
        let (lines, bytes) = res.unwrap();
        assert_eq!(lines, 1);
        assert_eq!(bytes, 24);
    }

    #[test]
    fn test_count_lines_twelve() {
        let res = count_lines_bytes("tests/inputs/twelve.txt");
        assert!(res.is_ok());
        let (lines, bytes) = res.unwrap();
        assert_eq!(lines, 12);
        assert_eq!(bytes, 63);
    }
}


