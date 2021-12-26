//! # *genpwd* - a Rust command line program
//! This program generates a randomized password from character classes. <br/>
//! The minmum length of generated passwords is 10. <br/>
//! Character classes are \[a-z\], \[A-Z\], \[0-9\] and special sign \!"$%&/?\\\(\)\{\}\[\]\+\*~#'<>|,;\.:\-_. 
//!
//! ## Information
//! * **Version:** 0.2.0
//! * **Documentation:** <a href="https://docs.rs/genpwd">generated documentation</a>
//! * **Supported OS:** Linux, Windows
//! * **Contact:** <a href="mailto:obr.rs@gmx.de">"Olaf Brozio" &lt;obr.rs@gmx.de&gt;</a>
//!
//! ## Rust version
//! Minimum version is 1.54; see: [clap](https://github.com/clap-rs/clap#Aspirations), [rand](https://github.com/rust-random/rand#rust-version-requirements)
//!
//! ## Dependencies
//!  * [clap](https://crates.io/crates/clap) - Minimum version is 2.34.0
//!  * [rand](https://crates.io/search?q=rand) - Minimum Version is 0.8.4
//!  
//! ## Usage
//! <pre><code>genpwd [OPTIONS]
//!
//! FLAGS:äö
//!    -h, --help                  Prints help information
//!    -V, --version               Prints version information<br/>
//! OPTIONS:
//!    -l, --length <value>        Minimal length of password [minimum: 10] [default: 12]
//!    -o, --occurrence <value>    Minimal occurrence of each character class [default: 2]
//! </code></pre>
//! ## Examples
//! <pre><code>&gt;genpwd
//! 9hL]0z.G&amp;r-O
//! </code></pre>
//! <pre><code>&gt;genpwd -o 4
//! z16N\6'8NkvyXK(<
//! </code></pre>
//! <pre><code>&gt;genpwd -l 20
//! 5X,qN~f02143c-2q6>]1
//! </code></pre>
//! <pre><code>&gt;genpwd -o 3 -l 14
//! *;4!7c0KHLdc:i
//! </code></pre>
//!
//! ## License
//! **genpwd** is distributed under the terms of both the MIT license and the Apache License (Version 2.0).<br/>
//! See the [LICENSE-APACHE](../../../LICENSE-APACHE) and [LICENSE-MIT](../../../LICENSE-MIT) files in this repository for more information.
//!
//! ## Changes for version 0.2.0
//! * README.md / documentation
//! * Help message uses parentheses instead of sqare brackets.
//! * Version of dependency **clap** is "^2.34.0".
//! * Version of dependency **rand** "^0.8.4".  
//!

extern crate clap;

mod config;
mod password;

use config::Minimum;
use password::generate;

#[doc(hidden)]
fn main() {
	let minimum = Minimum::new();
	println!("{}", generate(&minimum));
}
