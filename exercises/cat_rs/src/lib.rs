use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    
    // Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    pub files: Vec<String>,
    
    // Number lines
    #[arg(
        short('n'),
        long("number"),
        conflicts_with("number_nonblank_lines")
    )]
    pub number_lines: bool,
    
    // Number non-blank lines
    #[arg(short('b'), long("number-nonblank"))]
    pub number_nonblank_lines: bool,
}
