fn main() {
    for arg in std::env::args().skip(1) {
        print!("{} " , arg);
    }
    println!("{}", "");
}

// TODO:
// - [x] Access inner property of Args
// - [x] Skip the first item in inner, the executable path
// - [x] Print the residual items separated by space
//  -> TODO: Remove appended space!
// - [?] Think implementing the flags (newline, no space...)
//    - "-n" flag avoids newline after our output
// Args.inner
