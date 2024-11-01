use std::fs;
use clap::{Parser, Arg, ArgAction, Command};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    files: Vec<String>,
    number_lines: bool ,
    number_nonblank_lines: bool,
}

fn get_args() -> Args {
    let matches = Command::new("cat_rs")
        .version("0.1.0")
        .author("Ken Youens-Clark ")
        .about("Rust version of `cat`")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank")
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .help("Number non-blank lines")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    Args {
        files: matches.get_many("files").unwrap().cloned().collect(),
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number_nonblank"),
    }
}


fn main() {
    let args = get_args();
    // println!("{args:#?}");

    for filename in args.files {
        let file: String = fs::read_to_string(filename).expect("File does not exist");
        if args.number_lines {


        else {
            println!("{}", file);
        }
    }
}
