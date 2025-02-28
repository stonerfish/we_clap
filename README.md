# WE_CLAP : Web Enabled Command Line Argument Parser

## we_clap
Pronounced "We clap", as in "Give myself a round of applause."

The goal is to be flexible, write your command line code once and it should be able to run anywhere!

<a href="https://github.com/stonerfish/we_clap_examples/tree/master/we_clap_egui_demo"><figure><img alt="Screencast of an egui app running in web page with popup alerts showing a variety of clap help messages." src="https://github.com/stonerfish/cliw_examples/blob/master/we-clap_demos/we-clap_egui_demo/we-clap_egui_demo.gif?raw=true" width="50%"><figcaption>we_clap_egui_demo</figcaption></figure></a>

## Why use we_clap?

You like to use [`clap`], but you want your program to also run on the web.
Find `we_clap` at [`crates.io`], or the [`we_clap repository`]. 
Also check out the [`examples`] and [`docs`].

## Why not use we_clap?
* If you are writing only for the web, you may not be wanting to use command line arguments.
* You might have another use of the url query string.
* You might be compiling to wasm but the framework you use provides [`ArgsOs`]
  and standard output.
* Your program doesn't need command line arguments
* You don't like to use clap.  ( Check out [`cliw`] )

## How to use we_clap? 

### we_clap_demo

#### Cargo.toml
``` toml
[package]
name = "we_clap_demo"
version = "0.1.0"
edition = "2021"

[dependencies]
# get clap powers
clap = { version = "4.5.30", features = ["derive"] }
# get web ability for clap 
we_clap = { version = "0.1.1" , features = ["web-alert"] }
```

#### main.rs
``` rust
use clap::Parser;
use we_clap::WeParser; // Wrapper for clap Parser

impl we_clap::WeParser for Opts {} // Implement web enabled parser for your struct

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about)]
pub struct Opts {
    /// A required string 
    #[arg(short, long)]
    pub words: String,

    /// An optional value 
    #[arg(short, long)]
    pub value: Option<f32>,
}

fn main() {
    // Like magic, this will work on native parsing the command line arguments,
    // or on the web parsing the url query string as if it were command line arguments,
    // providing clap help and error messages to stdout/stderr on native or a popup alert on web/wasm.

    // use web enabled parse and it works on native or web.
    let opts: Opts = Opts::we_parse();  // Type annotations needed

    // this app doesn't do anything, except parse arguments and demonstrate clap powers in the web.
}
```
#### Running with wasm-server-runner.
``` console
cargo r --target wasm32-unknown-unknown
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `wasm-server-runner /path/to/target/wasm32-unknown-unknown/debug/we_clap_demo.wasm`
 INFO wasm_server_runner: uncompressed wasm output is 2.10mb in size
 INFO wasm_server_runner: starting webserver at http://127.0.0.1:1334
```

<a href="https://github.com/stonerfish/we_clap_examples/tree/master/we_clap_demo"><figure><img alt="Screencast of the we_clap_demo program running in web page with popup alerts showing a variety of clap help messages." src="https://github.com/stonerfish/cliw_examples/blob/master/we-clap_demos/we-clap_demo/we-clap_demo.gif?raw=true" width="50%"><figcaption>we_clap_demo</figcaption></figure></a>

## How does we_clap do it?

We_clap has two traits [`WeParser`] and [`WeCommand`] that wrap the [`clap::Parser`]
trait and the [`clap::Command`] struct.  These traits have wrapper functions that you call to
fill your derive struct or create your command, ie. [`we_parse()`] and [`we_get_matches()`].
These wrappers get arguments from [`ArgsOs`] on native and [`UrlArgs`] on the web.  These functions
handle error and help output like clap on native, on the web the error and help messages are
to the [`console`] or popup [`alert`].  See the [`docs`] and [`examples`] to learn how easy
it is to use.

### What is UrlArgs

[`UrlArgs`] is an iterator.  It splits the url into pieces.

 * Zeroith arg is the url path.
 * The rest of the args, if any, are the full [`decoded`] query strings.\
   The query strings start after the first '?' character and are delimited by '&'
 * Anything after a hash '#' is discarded.

### UrlArgs Example
```rust
    let url = "http:///www.example.org/index.html?--first&second&third#discard".to_string();
    let mut args : cliw::url_args::UrlArgs = url.into();
    assert_eq!(args.next().unwrap(),"http:///www.example.org/index.html");
    assert_eq!(args.next().unwrap(),"--first");
    assert_eq!(args.next().unwrap(),"second");
    assert_eq!(args.next().unwrap(),"third");
    assert_eq!(args.next(),None);
```


## We_clap Features

Web output functionality of the we_clap crate is gated by features.

* web-alert
    - Enable output to browser popup alert.
* web-console
    - Enable output to browser console.
    - Set by default

## License

Copyright 2025 Richard Gould

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[`alert`]: https://developer.mozilla.org/en-US/docs/Web/API/Window/alert
[`ArgsOs`]: https://doc.rust-lang.org/std/env/struct.ArgsOs.html
[`clap`]: https://crates.io/crates/clap
[`clap::Parser`]: https://docs.rs/clap/latest/clap/trait.Parser.html
[`clap::Command`]: https://docs.rs/clap/latest/clap/struct.Command.html
[`cliw`]: https://crates.io/crates/cliw
[`console`]: https://developer.mozilla.org/en-US/docs/Web/API/console
[`crates.io`]: http://crates.io/crates/we_clap
[`docs`]: https://docs.rs/we_clap
[`examples`]: https://github.com/stonerfish/we_clap_examples
[`UrlArgs`]: https://docs.rs/cliw/latest/cliw/url_args/struct.UrlArgs.html
[`we_clap`]: https://github.com/stonerfish/clap/tree/we_clap
[`we_clap_demo`]: https://github.com/stonerfish/we_clap_examples/tree/master/we_clap_demo
[`we_clap repository`]: https://github.com/stonerfish/we_clap
[`WeCommand`]: https://docs.rs/we_clap/latest/we_clap/struct.WeCommand.html
[`we_get_matches()`]: https://docs.rs/we_clap/latest/we_clap/struct.WeCommand.html#method.we_get_matches
[`we_parse()`]: https://docs.rs/we_clap/latest/we_clap/WeParse/fn.we_parse.html
[`WeParser`]: https://docs.rs/we_clap/latest/we_clap/trait.WeParser.html
