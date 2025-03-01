#![warn(missing_docs)]
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::doc_markdown)]
//! # WE_CLAP : Web Enabled Command Line Argument Parser
//!
//! # we_clap
//! Pronounced "We clap", as in "Give yourself a round of applause."
//!
//! The goal is to be flexible, write your command line code once and it
//! should be able to run anywhere!
//!
//! We_clap has two traits, [`WeCommand`] and [`WeParser`] that wrap the
//! [`clap::Command`] struct and the [`clap::Parser`] traits.  These traits
//! have wrapper functions that you call to fill your derive struct or work
//! with your command.  For example; [`WeCommand::we_get_matches()`]
//! and [`WeParser::we_parse()`].  These wrappers get arguments from
//! [`ArgsOs`] on native and [`UrlArgs`] on the web.  They also handle error
//! and help output like clap on native, on the web the error and help
//! messages are to the [`console`] or popup [`alert`].
//!
//! # We_clap Features
//!
//! Web output functionality of the we_clap crate is gated by features.
//!
//! * web-alert
//!     - Enable output to browser popup alert.
//! * web-console
//!     - Enable output to browser console.
//!     - Set by default
//!
//! # Example
//! ## we_clap_demo
//!
//! ### Cargo.toml
//! ``` toml
//! [package]
//! name = "we_clap_demo"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! # get clap powers
//! clap = { version = "4.5.30", features = ["derive"] }
//! # get web ability for clap
//! we_clap = { version = "0.1.1" , features = ["web-alert"] }
//! ```
//!
//! ### main.rs
//! ``` rust
//! use clap::Parser;
//! use we_clap::WeParser; // Wrapper for clap Parser
//!
//! // Implement web enabled parser for your struct
//! impl we_clap::WeParser for Opts {}
//!
//! #[derive(Parser, Debug, Default)]
//! #[command(author, version, about, long_about)]
//! pub struct Opts {
//!     /// An optional value
//!     #[arg(short, long)]
//!     pub value: Option<f32>,
//! }
//!
//! fn main() {
//! // Like magic, using we_parse() will work on native parsing
//! // the command line arguments or on the web parsing the url query
//! // string as if it were command line arguments, providing clap help
//! // and error messages to stdout/stderr on native or a popup alert
//! // on web/wasm.
//!
//!     // Type annotations needed
//!     let opts: Opts = Opts::we_parse();
//!
//!     // this app doesn't do anything, except parse arguments and
//!     // demonstrate clap powers in the web.
//! }
//! ```
//!
//! [`alert`]: https://developer.mozilla.org/en-US/docs/Web/API/Window/alert
//! [`ArgsOs`]: https://doc.rust-lang.org/std/env/struct.ArgsOs.html
//! [`clap::Command`]: https://docs.rs/clap/latest/clap/struct.Command.html
//! [`clap::Parser`]: https://docs.rs/clap/latest/clap/trait.Parser.html
//! [`console`]: https://developer.mozilla.org/en-US/docs/Web/API/console
//! [`UrlArgs`]: https://docs.rs/cliw/latest/cliw/url_args/struct.UrlArgs.html

use clap::{error, ArgMatches, Command, Parser};

#[cfg(target_arch = "wasm32")]
use clap::error::ErrorKind;

#[cfg(target_arch = "wasm32")]
use cliw::url_args::UrlArgs;

/// # Wrapper trait implemented for [`clap::Command`]
///
/// Functions to work with your [`clap::Command`] on native or the web.\
/// On native the arguments come from [`std::env::ArgsOs`].\
/// On the web the arguments come from [`cliw::url_args::UrlArgs`].
///
/// Also some functions to print the help and long help.
///
/// Use these instead of their clap counterparts to make your command
/// line program work on native or the web.
///
/// # Example
/// ``` rust
/// use clap::Command; // Use clap to parse the arguments
/// use we_clap::WeCommand; // Use we_clap to provide the arguments to clap.
///
/// // Create your clap command.
/// let mut cli = Command::new("Native and Web Program");
///
/// // Use WeCommand function instead of get_matches.
/// let matches = &cli.we_get_matches();
/// ```
pub trait WeCommand {
    /// # Wrapper for [`clap::Command::get_matches()`]
    ///
    /// Gets command line arguments on native or the web.\
    /// Native args are from [`std::env::ArgsOs`].\
    /// Web args are from [`cliw::url_args::UrlArgs`].\
    ///
    /// # Panics
    ///
    /// May panic if contradictory arguments or settings exist (debug builds).
    /// This is normal clap behaviour.
    ///
    /// # Exit
    ///
    /// This functon may call [`std::process::exit()`] after printing messages
    /// if command line arguments are wrong or a help or version type argument
    /// is given.  This is normal clap behaviour.
    ///
    /// # Example
    /// ``` rust
    /// use clap::Command; // Use clap to parse the arguments.
    /// use we_clap::WeCommand; // Use we_clap to provide the arguments.
    ///
    /// let mut cli = Command::new("Native and Web Program");
    ///
    /// // Use WeCommand function instead of clap get_matches
    /// let matches = &cli.we_get_matches();
    /// ```
    #[must_use]
    fn we_get_matches(self) -> ArgMatches;

    /// # Wrapper for [`clap::Command::get_matches_mut()`]
    ///
    /// Like [`we_get_matches`](crate::WeCommand::we_get_matches()) but
    /// doesn't consume the `Command`.
    ///
    /// Gets command line arguments on native or the web.\
    /// Native args are from [`std::env::ArgsOs`].\
    /// Web args are from [`cliw::url_args::UrlArgs`].\
    ///
    /// # Panics
    ///
    /// May panic if contradictory arguments or settings exist (debug builds).
    /// This is normal clap behaviour.
    ///
    /// # Exit
    ///
    /// This functon may call [`std::process::exit()`] after printing messages if
    /// command line arguments are wrong or a help or version type argument is given.
    /// This is normal clap behaviour.
    ///
    /// # Example
    /// ``` rust
    /// use clap::Command; // Use clap to parse the arguments
    /// use we_clap::WeCommand; // Use we_clap to provide the arguments to clap.
    ///
    /// let mut cli = Command::new("Native and Web Program");
    ///
    /// // use WeCommand function instead of clap get_matches_mut
    /// let matches = cli.we_get_matches_mut(); // use WeCommand function instead of get_matches
    /// ```
    #[must_use]
    fn we_get_matches_mut(&mut self) -> ArgMatches;

    /// # Wrapper for [`clap::Command::try_get_matches()`]
    ///
    /// Gets command line arguments on native or the web.\
    /// Native args are from [`std::env::ArgsOs`].\
    /// Web args are from [`cliw::url_args::UrlArgs`].\
    ///
    /// # Panics
    ///
    /// May panic if contradictory arguments or settings exist (debug builds).
    /// This is normal clap behaviour.
    ///
    /// # Errors
    ///
    /// If help or version type arguments are entered than [`clap::error`]
    /// will be returned instead of [`clap::ArgMatches`].
    /// This is normal clap behaviour.
    ///
    /// # Example
    ///
    /// ``` rust
    /// use clap::Command; // Use clap to parse the arguments
    /// use we_clap::WeCommand; // Use we_clap to provide the arguments.
    ///
    /// let mut cli = Command::new("Native and Web Program");
    ///
    /// // use WeCommand function instead of clap try_get_matches
    /// match cli.we_try_get_matches() {
    ///     Ok(cli) => {} // handle cli
    ///     Err(err) => {} // handle error
    /// }
    /// ```
    fn we_try_get_matches(self) -> error::Result<ArgMatches>;

    /// # Print help message
    /// Prints a help message on native or the web.\
    /// Native output is to stdout/stderr.\
    /// Web output is to console or popup alert
    ///
    /// # Errors
    /// On native a [`Result`] is returned from
    /// [`clap::Command::print_help()`].  This is normal clap behaviour.
    ///
    /// On the web ouput errors are ignored.  Ok(()) is always returned
    ///
    /// # Example
    /// ``` rust
    /// use clap::Command; // Use clap to parse the arguments
    /// use we_clap::WeCommand; // Use we_clap to provide the arguments to clap.
    ///
    /// let mut cli = Command::new("Native and Web Program");
    ///
    /// // use WeCommand print function instead of clap print_help
    /// let result = cli.we_print_help();
    /// ```
    fn we_print_help(&mut self) -> std::io::Result<()>;

    /// # Print long help message
    /// Prints a long help message on native or the web.\
    /// Native output is to stdout/stderr.\
    /// Web output is to console or popup alert
    ///
    /// # Errors
    /// On native the [`Result`] is returned from
    /// [`clap::Command::print_help()`].   This is normal clap behaviour.
    ///
    /// On the web ouput errors are ignored.  Ok(()) is always returned
    ///
    /// # Example
    /// ``` rust
    /// use clap::Command; // Use clap to parse the arguments
    /// use we_clap::WeCommand; // Use we_clap to provide the arguments.
    ///
    /// let mut cli = Command::new("Native and Web Program");
    ///
    /// // use WeCommand print function instead of clap print_help
    /// let result = cli.we_print_long_help();
    /// ```
    fn we_print_long_help(&mut self) -> std::io::Result<()>;
}

impl WeCommand for Command {
    fn we_get_matches(self) -> ArgMatches {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.get_matches()
        }
        #[cfg(target_arch = "wasm32")]
        {
            let command = self.try_get_matches_from(UrlArgs::new());
            match command {
                Ok(command) => command,
                Err(err) => {
                    let msg = format!("{err}");
                    match err.kind() {
                        ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => {
                            cliw::output::print(&msg);
                        }
                        _ => {
                            cliw::output::eprint(&msg);
                        }
                    }
                    std::process::exit(0); // Exit code meaningless on wasm.
                }
            }
        }
    }
    fn we_get_matches_mut(&mut self) -> ArgMatches {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.get_matches_mut()
        }
        #[cfg(target_arch = "wasm32")]
        {
            let command = self.try_get_matches_from_mut(UrlArgs::new());
            match command {
                Ok(command) => command,
                Err(err) => {
                    let msg = format!("{err}");
                    match err.kind() {
                        ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => {
                            cliw::output::print(&msg);
                        }
                        _ => {
                            cliw::output::eprint(&msg);
                        }
                    }
                    std::process::exit(0); // Exit code meaningless on wasm.
                }
            }
        }
    }
    fn we_try_get_matches(self) -> error::Result<ArgMatches> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.try_get_matches()
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.try_get_matches_from(UrlArgs::new())
        }
    }

    fn we_print_help(&mut self) -> std::io::Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.print_help()
        }

        #[cfg(target_arch = "wasm32")]
        {
            let styled = &self.render_help();
            cliw::output::print(&format!("{styled}"));
            Ok(())
        }
    }

    fn we_print_long_help(&mut self) -> std::io::Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.print_long_help()
        }

        #[cfg(target_arch = "wasm32")]
        {
            let styled = &self.render_long_help();
            cliw::output::print(&format!("{styled}"));
            Ok(())
        }
    }
}

/// # Wrapper trait for [`clap::Parser`]
///
/// Functions to parse your Opt with clap on native or the web.\
/// On native the arguments come from [`std::env::ArgsOs`].\
/// On the web the arguments come from [`cliw::url_args::UrlArgs`].
///
/// Use these instead of their clap counterparts to make your command
/// line program work on native or the web.
///
/// # Example
/// ``` rust
/// use clap::Parser; // Use clap to parse the arguments
/// use we_clap::WeParser; // Use we_clap to provide the arguments.
///
/// #[derive(Parser, Debug, Default)]
/// #[command(author, version, about, long_about)]
/// pub struct Opts {}
///
/// // Implement web enabled parser for your struct
/// impl we_clap::WeParser for Opts {}
///
/// // Use we_parse and it works on native or web.
/// // Type annotations needed.
/// let opts: Opts = Opts::we_parse();
/// ```
pub trait WeParser {
    /// # Wrapper for [`clap::Parser::parse()`]
    ///
    /// Gets command line arguments on native or the web.\
    /// Native args are from [`std::env::ArgsOs`].\
    /// Web args are from [`cliw::url_args::UrlArgs`].\
    ///
    /// # Panics
    ///
    /// May panic if contradictory arguments or settings exist (debug builds).
    /// This is normal clap behaviour.
    ///
    /// # Exit
    ///
    /// This functon may call [`std::process::exit()`] after printing messages if
    /// command line arguments are wrong or a help or version type argument is given.
    /// This is normal clap behaviour.
    ///
    /// # Example
    /// ``` rust
    /// use clap::Parser; // Use clap to parse the arguments
    /// use we_clap::WeParser; // Use we_clap to provide the arguments to clap.
    ///
    /// #[derive(Parser, Debug, Default)]
    /// #[command(author, version, about, long_about)]
    /// pub struct Opts {}
    ///
    /// // Implement web enabled parser for your struct
    /// impl we_clap::WeParser for Opts {}
    ///
    /// // use we_parse and it works on native or web.
    /// // Type annotations needed
    /// let opts: Opts = Opts::we_parse();
    /// ```
    #[must_use]
    fn we_parse<T>() -> T
    where
        T: Parser,
    {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Parser::parse()
        }
        #[cfg(target_arch = "wasm32")]
        {
            let opts = Parser::try_parse_from(UrlArgs::new());
            match opts {
                Ok(opts) => opts,
                Err(err) => {
                    let msg = format!("{err}");
                    match err.kind() {
                        ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => {
                            cliw::output::print(&msg);
                        }
                        _ => {
                            cliw::output::eprint(&msg);
                        }
                    }

                    std::process::exit(0); // Exit code meaningless on wasm.
                }
            }
        }
    }
    /// # Wrapper for [`clap::Parser::try_parse()`]
    ///
    /// Gets command line arguments on native or the web.\
    /// Native args are from [`std::env::ArgsOs`].\
    /// Web args are from [`cliw::url_args::UrlArgs`].\
    ///
    /// # Panics
    ///
    /// May panic if contradictory arguments or settings exist (debug builds).
    /// This is normal clap behaviour.
    ///
    /// # Errors
    ///
    /// If help or version type arguments are entered than [`clap::error`]
    /// will be returned.  This is normal clap behaviour.
    ///
    /// # Example
    /// ``` rust
    /// use clap::Parser; // Use clap to parse the arguments
    /// use we_clap::WeParser; // Use we_clap to provide the arguments.
    ///
    /// #[derive(Parser, Debug, Default)]
    /// #[command(author, version, about, long_about)]
    /// pub struct Opts {}
    ///
    /// // Implement web enabled parser for your struct
    /// impl we_clap::WeParser for Opts {}
    ///
    /// // use we_parse and it works on native or web.
    /// // Type annotations needed
    /// match Opts::we_try_parse::<Opts>() {
    ///     Ok(opts) => {} // handle opts
    ///     Err(err) => {} // handle error
    /// }
    fn we_try_parse<T>() -> Result<T, error::Error>
    where
        T: Parser,
    {
        #[cfg(target_arch = "wasm32")]
        {
            Parser::try_parse_from(UrlArgs::new())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Parser::try_parse()
        }
    }
}

#[cfg(test)]
mod tests {
    //   use super::*;
    //   lazy: let us only use what we need.
    //   Or maybe we do need to use everything because we test everything ?
    /*
        #[test]
        fn it_works() {
            let result = add(2, 2);
            assert_eq!(result, 4);
        }
    */
}
