use std::env;
use std::fs;
use clap::Parser;
use std::cmp::min;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        value_name = "FILE",
        default_value = "-"
    )]
    files: Vec<String>,

    #[arg(
        short('n'),
        long,
        default_value = "10",
        value_name="LINES",
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    lines: u64,

    #[arg(
        short('c'),
        long,
        value_name="BYTES",
        conflicts_with("lines"),
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    bytes: Option<u64>,
}


fn main() {
   let args = Args::parse();
   //let args: Vec<String> = env::args().collect();
   println!("{:?}", args);

   for filename in &args.files {
       // Open and read from a File
       let lines: Vec<String> = fs::read_to_string(&filename)
                .unwrap()
                .lines()
                .map(String::from)
                .collect();

       // Print file contents
       println!(
           "{}==> {filename} <==",
           if args.files.len() > 1 { "\n" } else { "" }
       );

       let limit = min(lines.len(), args.lines as usize);
       for i in 0..limit {
            println!("{}", lines[i]);
       }
   }
}
