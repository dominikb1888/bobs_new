
## Immediate Steps:

1. Rust Types
2. Understand how to open a file with Rust and read it line-by-line
3. Copy over the code from Echo to print the contents again
4. Think about features we would want: e.g. line numbering
5. Create a proper argument parser with clap
6. Understand STDIN, STDOUT, STDERR: https://www.howtogeek.com/435903/what-are-stdin-stdout-and-stderr-on-linux/
7. Think about reading a file line-by-line: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
8. Applying this to our program using a BufReader instead of Read to read line-by-line: https://doc.rust-lang.org/std/io/struct.BufReader.html

## Advanced:

- Why we use anyhow::Result<()>: https://www.reddit.com/r/rust/comments/17neomp/result_boxdyn_error_vs_anyhowresult/
- https://www.linkedin.com/pulse/demystifying-anyhow-rust-amit-nadiger-eiwbc/
