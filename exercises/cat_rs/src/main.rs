use std::fs::File;
use std::io::Read;

use clap::Parser;
use anyhow::Result;


/// Rust version of `cat`


#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    
    // Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    
    // Number lines
    #[arg(
        short('n'),
        long("number"),
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,
    
    // Number non-blank lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    // println!("{:?}", args);

    // Loop through args.files Vector
    for filepath in args.files {
        // // check if file exists
        
        // // if so, open file
        let mut file = File::open(filepath)?;
        let mut contents = String::new();
    
        // // // read lines
        // // // store in Vector<String>
        file.read_to_string(&mut contents)?;
        contents = contents.trim_end().to_string();

        let lines: Vec<&str> = contents.split("\n").collect();

        if args.number_lines || args.number_nonblank_lines {
            let mut counter = 0;
            for (no, line) in lines.into_iter().enumerate() {
                let mut linenumber = no + 1;
                
                if args.number_lines && line == "" {
                    counter += 1;
                }

                if args.number_lines {
                    linenumber = linenumber - counter;
                }

                print!("{:>6}\t{}\n", 
                       if line == "" && args.number_lines { "\n".to_string() } else { linenumber.to_string() }, 
                       line
                )            
            }
        } else {
            println!("{contents}")
        }

        // // close file
        // // add a file separator to our lines vector to separate files (e.g. ---------- FILENAME )
        // // 
    }
    
    Ok(())
    // print lines vector to stdout
}
