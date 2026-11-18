use gumdrop::Options;
//use std::env;
use std::fs;
//use std::process;
#[derive(Debug, Options)]
struct Args {
    #[options(short = "n", long = "number")]
    number_all_lines: bool,
    #[options(short = "b", long = "number-nonblank")]
    number_nonblank_lines: bool,
    #[options(free)]
    filenames: Vec<String>,
}
//helper function that takes 3 arguemnts: input text and two flags
//prints the whole input text or with line numbering depending on the flags
fn print_contents(contents: &str, number_all_lines: bool, number_nonblank_lines: bool){
    //start a fresh line counter at 0 for each file before the loops
    let mut counter = 0;
    if number_all_lines{
        //number every line including empty lines
        for line in contents.lines(){
            counter += 1;
            //right align the number in six character field then separating with a tab from text
            println!("{:>6}\t{}", counter, line);
        }
    }
    else if number_nonblank_lines{
        for line in contents.lines(){
            //skips numbering empty lines without counting or numbering them
            if line.is_empty(){
                println!();
                continue;
            }
            counter += 1;
            println!("{:>6}\t{}", counter, line);
        }
    }
    else{
        print!("{contents}");
    }
}
fn main() {
    let args = Args::parse_args_default_or_exit();
    //dbg!(&args);
    for filename in &args.filenames {
        let contents = fs::read_to_string(filename);
        match contents{
            Ok(contents) => print_contents(
                &contents,
                args.number_all_lines,
                args.number_nonblank_lines,
            ),
            Err(err) => eprintln!("Unable to open {filename}: {err}"),
        }
    }
}

