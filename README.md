# genpwd - a Rust command line program
This program generates a randomized password from character classes.

The minmum length is 10.  
Character classes are [a-z], [A-Z], [0-9] and special sign.

## usage
<pre><code>genpwd [OPTIONS]

FLAGS:
   -h, --help                  Prints help information
   -V, --version               Prints version information<br/>
OPTIONS:
   -l, --length <value>        Minimal length of password [minimum: 10] [default: 12]
   -o, --occurrence <value>    Minimal occurrence of each character class [default: 2]
</code></pre>
## examples
&gt;genpwd  
9hL]0z.G&amp;r-O

&gt;genpwd -o 4  
z16N\6'8NkvyXK(<

&gt;genpwd -l 20  
5X,qN~f02143c-2q6>]1

&gt;genpwd -o 3 -l 14  
*;4!7c0KHLdc:i

## License
genpwd is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See the [LICENSE-APACHE](https://github.com/obr-rs/genpwd/blob/master/LICENSE-APACHE) and [LICENSE-MIT](https://github.com/obr-rs/genpwd/blob/master/LICENSE-MIT) files in this repository for more information.