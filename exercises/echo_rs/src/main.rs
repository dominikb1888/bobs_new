fn main() {
    let mut output: Vec<String> = Vec::new();
    let mut no_newline = false; // removes newline from the end of the output
    let mut delimiter = " "; // sets the delimiter to be space or none

    let args = std::env::args().skip(1);

    // separating arguments (aka context) from content
    // echo hello -n world this is -d great
    for arg in args { // String
        match arg.as_str() { // String
            "-n" => no_newline = true,
            "-d" => delimiter = "",
            _    => output.push(arg),
        };
    }
    // no_newline: true
    // delimiter: ""
    // output: ["hello", "world", "this", "is", "great"]

    if output.is_empty() {
        eprint!("{}", "missing required free argument");
        std::process::exit(1)
    }

    let mut stringed_list = output.join(delimiter);

    if no_newline == true {
        print!("{}", stringed_list);
    }
    else {
        println!("{}", stringed_list);
    }
}

