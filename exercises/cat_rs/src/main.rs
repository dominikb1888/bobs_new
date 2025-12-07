use std::fs::File;
use std::io::Read;

use clap::Parser;
use anyhow::Result;

use cat_rs::Args;


fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> { 
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
