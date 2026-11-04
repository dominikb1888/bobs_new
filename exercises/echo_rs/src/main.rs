use gumdrop::Options;

#[derive(Debug, Options)]
struct Args {

    #[options(short = "n")]
    no_newline: bool, // define a boolean argument, set default

    #[options(short = "d", default=" ")]
    delimiter: String, // define a String argument, set default

    #[options(free, help="missing required free argument")]
    output: Vec<String>,

}

fn main() {
    let args = Args::parse_args_default_or_exit();
    let mut stringed_list = args.output.join(args.delimiter.as_str());

    if args.no_newline == true {
        print!("{}", stringed_list);
    } else {
        println!("{}", stringed_list);
    }
}

