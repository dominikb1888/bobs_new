use gumdrop::Options;

#[derive(Debug, Options)]
struct Args {

    #[options(short = "n")]
    no_newline: bool; // define a boolean argument, set default

    #[options(short = "d")]
    delimiter: String; // define a String argument, set default

    #[options(free)]
    output: Vec<String>;

}




fn main() {
    // let mut output: Vec<String> = Vec::new();
    // let mut no_newline = false; // removes newline from the end of the output
    // let mut delimiter = " "; // sets the delimiter to be space or none

    let args = Args::parse_args_default_or_exit();

    // separating arguments (aka context) from content
    // echo hello -n world this is -d great
    // for arg in args { // String
    //    match arg.as_str() { // String
    //        "-n" => no_newline = true,
    //        "-d" => delimiter = "",
    //        _    => output.push(arg),
    //    };
    // }
    // no_newline: true
    // delimiter: ""
    // output: ["hello", "world", "this", "is", "great"]

    // if output.is_empty() {
    //    eprint!("{}", "missing required free argument");
    //    std::process::exit(1)
    // }

    let mut stringed_list = args.output.join(args.delimiter);

    if args.no_newline == true {
        print!("{}", stringed_list);
    }
    else {
        println!("{}", stringed_list);
    }
}

