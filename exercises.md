# Exercises

{: .highlight .k }
>
> **Required Reading:** [Command-Line Rust](https://ebookcentral.proquest.com/lib/th-deggendorf/reader.action?docID=6853886)
>

>
> Recommended Reading:
> - [Practical Systems Programming with Rust](https://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6447686)
> - [The Linux Programming Interface](https://www.man7.org/tlpi/)
>

## 1. Truth or Consequences

Objectives:
- Getting Started with "Hello World"
- **Exercise**{: .label .label-red} [Hello World](https://classroom.github.com/a/cYd-7xPR)
- Code Organization in Rust
- Creating and Running a Project with Cargo
- Writing and Running Integration Tests

- Creating our First Program "true" (https://man7.org/linux/man-pages/man1/true.1.html)

Linux Exit codes:

| Exit code | 	Meaning of the code |
| --- | --- |
| 0	| Command executed with no errors |
| 1	| Code for generic errors |
| 2	| Incorrect command (or argument) usage |
| 126	| Permission denied (or) unable to execute |
| 127	| Command not found, or PATH error |
| 128+n	| Command terminated externally by passing signals, or it encountered a fatal error |
| 130	| Termination by Ctrl+C or SIGINT (termination code 2 or keyboard interrupt) |
| 143	| Termination by SIGTERM (default termination) |
| 255/*	| Exit code exceeded the range 0-255, hence wrapped up |


## 2. Test for Echo

Objectives:
- Process command-line arguments with the clap crate
- Use Rust types like strings, vectors, slices, and the unit type
- Use expressions like match, if, and return
- Use Option variants to represent Some value or None
- Handle errors using the Result variants of Ok and Err
- Understand the difference between stack and heap memory
- Test for text that is printed to STDOUT and STDERR
- Use Iterator::collect to turn an iterator into a vector
- Create a struct


## 3. On the Catwalk

Objectives:
- Use testing-first development
- Test for the existence of a file
- Create a random string for a filename that does not exist
- Read regular files or STDIN (pronounced standard in )
- Use eprintln! to print to STDERR and format! to format a string • Write a test that provides input on STDIN
- Define mutually exclusive arguments
- Use the enumerate method of an iterator


## 4. Head Aches

Objectives:
- Create optional command-line arguments that accept numeric values
- Convert between types using as
- Use take on an iterator or a filehandle
- Preserve line endings while reading a filehandle
- Read bytes versus characters from a filehandle
- Use the turbofish operator


## 5. Word to Your Mother

Objectives:
- Use the Iterator::all function
- Create a module for unit tests
- Fake a filehandle for testing
- Conditionally format and print a value
- Conditionally compile a module when testing
- Break a line of text into words, bytes, and characters


## 6. Den of Uniquity

Objectives:
- Write to a file or STDOUT
- Use a closure to capture a variable
- Apply the don’t repeat yourself (DRY) concept
- Use the Write trait and the write! and writeln!
- Use temporary files
- Indicate the lifetime of a variable macros


## 7. Finders Keepers

Objectives:
- Use clap to constrain possible values for command-line arguments
- Use a regular expression to find a pattern of text
- Create an enumerated type with an implementation
- Recursively search filepaths using the walkdir crate
- Use the Iterator::any function
- Chain multiple filter, map ,and filter_map operations
- Compile code conditionally when on Windows or not
- Refactor code


## 8. Shave and a Haircut

Objectives:
- Read and write a delimited text file using the
- Deference a value using *
- Use Iterator::flatten
- Use Iterator::flat_map csv crate to remove nested structures from iterators to combine Iterator::map and Iterator::flatten


## 9. Jack the Grepper

Objectives:
- Using a case-sensitive regular expression
- Variations of regular expression syntax
- Another syntax to indicate a trait bound
- Using Rust’s bitwise exclusive-OR operator


## 10. Boston Commons

Objectives:
- Manually iterate the lines of a filehandle using
- match on combinations of possibilities using a tuple
- Use std::cmp::Ordering when comparing strings Iterator::next


## 11. Tailor Swyfte

Objectives:
- Initialize a static, global, computed value
- Seek to a line or byte position in a filehandle
- Indicate multiple trait bounds on a type using the where
- Build a release binary with Cargo
- Benchmark programs to compare runtime performance clause


## 12. Fortunate Son

Objectives:
- Use the Path and PathBuf structs to represent system paths
- Parse records of text spanning multiple lines from a file
- Use randomness and control it with seeds
- Use the OsStr and OsString types to represent filenames
- (Optional): Basic async processing using Tokio (https://tokio.rs/tokio/tutorial)


## 13. Rascalry

Objectives:
- Find today’s date and do basic date manipulations
- Use Vec::chunks to create groupings of items
- Combine elements from multiple iterators
- Produce highlighted text in the terminal


## 14. Elless Island

Objectives:
- Find today’s date and do basic date manipulations
- Use Vec::chunks to create groupings of items
- Combine elements from multiple iterators
- Produce highlighted text in the terminal
