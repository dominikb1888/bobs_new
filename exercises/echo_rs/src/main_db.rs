use gumdrop::Options;


#[derive(Debug, Options)]
struct Opts {

    #[options(short = "n", long = "no-newline", help = "Do not print a newline")]
    no_newline: bool,

    #[options(short = "d", long = "no-space", help = "Do not put a space between free args")]
    no_space: bool,

    #[options(free, required, help = "TEXT")]
    positional_args: Vec<String>,

}


fn main() {
    let opts = Opts::parse_args_default_or_exit();
    let mut delimiter = " ";

    if opts.no_space {
        delimiter = "";
    }

    let text: String = opts.positional_args.join(delimiter);

    if opts.no_newline {
        print!("{}", text)
    } else {
        println!("{}", text)
    }
}

// TODO:
// - [x] Access inner property of Args
// - [x] Skip the first item in inner, the executable path
// - [x] Print the residual items separated by space
//  -> TODO: Remove appended space!
// - [?] Think implementing the flags (newline, no space...)
//    - "-n" flag avoids newline after our output
// Args.inner
