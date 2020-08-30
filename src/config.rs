use clap::*;

/// Contains minimal requirement for generating passwords
pub struct Minimum {
    /// Minimal occurrence of each character class
    pub occurrence: u16,
    /// Minimal length of password
    pub length: usize,
}

impl Minimum {
    /// Creates a new instance using command line argument for minimal requirements
    pub fn new() -> Minimum {
        let min_occurrence = 0;
        let min_length = 10;
        let matches = get_arg_matches();
        let arg_occurrence = value_t!(matches.value_of("occurrence"), u16);
        let arg_length = value_t!(matches.value_of("length"), usize);
        Minimum {
            occurrence: match arg_occurrence {
                Ok(val_occurrence) => val_occurrence,
                _ => min_occurrence,
            },
            length: match arg_length {
                Ok(val_length) => {
                    if val_length < min_length {
                        min_length
                    } else {
                        val_length
                    }
                }
                _ => min_length,
            },
        }
    }
}

/// Returns all matching arguments from command line
fn get_arg_matches() -> ArgMatches<'static> {
    App::new(env!("CARGO_CRATE_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("occurrence")
                .short("o")
                .long("occurrence")
                .takes_value(true)
                .value_name("value")
                .default_value("2")
                .help("Minimal occurrence of each character class [default: 2]"),
        )
        .arg(
            Arg::with_name("length")
                .short("l")
                .long("length")
                .takes_value(true)
                .value_name("value")
                .default_value("12")
                .help("Minimal length of password [minimum: 10] [default: 12]"),
        )
        .get_matches()
}
