use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Args {

    // Input file(s)
    #[options(free)]
    pub filenames: Vec<String>,

    // Number lines
    #[options(short = "n")]
    pub numbered: bool,

    // Number non-blank lines
    #[options(short = "b")]
    pub blank: bool,
}

// 2. The Internal Logic (The Behavior)
pub enum Numbering {
    None,
    All,
    NonBlank,
}


pub struct Config {
    pub numbering: Numbering,
    pub capitaliztion: Capitalization,
}

// The Bridge (Sanitization Pattern)
impl From<&Args> for Config {
    fn from(args: &Args) -> Self {
        let numbering = if args.blank {
            Numbering::NonBlank
        } else if args.numbered {
            Numbering::All
        } else {
            Numbering::None
        };

        Config { numbering: numbering, capitalization: Capitalization }
    }
}
