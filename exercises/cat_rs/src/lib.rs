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
        conflicts_with("number_nonblank")
    )]
    pub number: bool,
    
    // Number non-blank lines
    #[arg(short('b'), long("number-nonblank"))]
    pub number_nonblank: bool,

    // Make everything uppercase
    #[arg(short('U'), long("uppercase"))]
    pub uppercase: bool,

    // Make everything uppercase
    #[arg(short('L'), long("lowercase"))]
    pub lowercase: bool,

}

// 2. The Internal Logic (The Behavior)
pub enum Numbering {
    None,
    All,
    NonBlank,
}

pub enum Case {
    None,
    Uppercase,
    Lowercase,
}

pub struct Config {
    pub numbering: Numbering,
    pub case: Case,
    // highlight: HighlightMode
    // git: GitMode
}

// 3. The Bridge (The Sanitization)
impl From<&Args> for Config {
    fn from(args: &Args) -> Self {
        let numbering = if args.number_nonblank {
            Numbering::NonBlank
        } else if args.number {
            Numbering::All
        } else {
            Numbering::None
        };

        let case = if args.uppercase {
            Case::Uppercase
        } else if args.lowercase {
            Case::Lowercase
        } else {
            Case::None
        };

        Config { numbering: numbering, case: case }
    }
}
