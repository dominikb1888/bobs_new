
//OLD CODE

    /*
    let output: Vec<String> = env::args().collect();
    let mut number_all_lines = false;
    let mut number_nonblank_lines = false;
    let mut fileNames: Vec<String> = Vec::new();

    for arg in output.iter().skip(1) {
        match arg.as_str() {
            "-n" | "--number" => number_all_lines = true,
            "-b" | "--number-nonblank" => number_nonblank_lines = true,
            _ => fileNames.push(arg.to_string()),
        }
    }

    for fileName in &fileNames {
        let file_path = fileName;
        let contents = fs::read_to_string(file_path);
        let mut counter = 0;

        match contents {
            Ok(contents) => {
                if number_all_lines {
                    for line in contents.lines() {
                        counter += 1;
                        println!("{:>6}\t{}", counter, line);
                    }
                } else if number_nonblank_lines {
                    for line in contents.lines() {
                        if line.is_empty() {
                            println!();
                            continue;
                        } else {
                            counter += 1;
                            println!("{:>6}\t{}", counter, line);
                        }
                    }
                } else {
                    print!("{contents}");
                }
            }
            Err(err) => {
                eprintln!("Unable to open {file_path}: {err}");
            }
        }
    }
}
