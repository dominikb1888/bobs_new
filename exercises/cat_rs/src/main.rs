use gumdrop::Options;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Options)]
struct Args {

    #[options(short = "n")]
    numbered: bool, // define a boolean argument, set default

    #[options(short = "b")]
    blank: bool,

    #[options(free, help="missing required free argument")]
    filenames: Vec<String>,

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse_args_default_or_exit();
    for filename in args.filenames {
        let mut file = File::open(&filename)?;
        let mut text = String::new();
        file.read_to_string(&mut text)?;

        match File::open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => {
                let mut blank_lines = 0;
                for (no, line) in text.lines().enumerate() {
                    if args.numbered {
                       println!("{:>6}\t{line}", no + 1);
                    } else if args.blank {
                        if line.is_empty() {
                            println!();
                         } else {
                            blank_lines += 1;
                            println!("{:>6}\t{}", blank_lines, line)
                         }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

