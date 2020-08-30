//! # genpwd - a Rust command line program
//! <p>This program generates a randomized password from character classes.</p>
//! <p>The minmum length is 10.<br/>
//! Character classes are [a-z], [A-Z], [0-9] and special sign.</p>
//! 
//! ## usage
//! <pre><code>genpwd [OPTIONS]
//! 
//! FLAGS:
//!    -h, --help                  Prints help information
//!    -V, --version               Prints version information<br/>
//! OPTIONS:
//!    -l, --length <value>        Minimal length of password [minimum: 10] [default: 12]
//!    -o, --occurrence <value>    Minimal occurrence of each character class [default: 2]
//! </code></pre>
//! ## examples
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
//! ## License
//! genpwd is distributed under the terms of both the MIT license and the Apache License (Version 2.0).
//!
//!See the [LICENSE-APACHE](https://github.com/obr-rs/genpwd/LICENSE-APACHE) and [LICENSE-MIT](https://github.com/obr-rs/genpwd/LICENSE-MIT) files in this repository for more information.
 

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
