/*!
## filtering
Use the ENV variable `RUST_LOG` with `module_name=level`

`RUST_LOG="tokio=warn,my_module=info,my_module::inner=trace"`

## optional features
* `time` allows formatting a UTC timestamp with the [`time`](time) crate.
    * see the formatting table [here](https://docs.rs/time/0.2.10/time/#formatting)
[time]: https://docs.rs/time
*/

#[cfg(all(doctest))]
doc_comment::doctest!("../README.md");

#[doc(inline)]
pub use termcolor::Color;

/// Initialize the logger
///
/// ```rust
/// use alto_logger::{
///     options::{ColorConfig, StyleConfig, TimeConfig},
///     Options,
///     TermLogger,
/// };
/// alto_logger::init(
///     TermLogger::new(
///         Options::default()
///             .with_style(StyleConfig::SingleLine)     // Default is a MultiLine output
///             .with_time(TimeConfig::relative_now())   // Default is no timestamp
///             .with_color(ColorConfig::only_levels()), // Default is full color
///     )
///     .unwrap(),
/// )
/// .unwrap()
/// ```
pub fn init(logger: impl log::Log + 'static) -> Result<(), Error> {
    // enables trace for all
    // TODO this is wrong
    log::set_max_level(log::LevelFilter::Trace);
    log::set_boxed_logger(Box::new(logger)).map_err(Error::SetLogger)
}

/// Convenience function to create a default terminal logger
pub fn init_term_logger() -> Result<(), Error> {
    TermLogger::new(Default::default()).and_then(init)
}

mod error;
mod filters;
mod loggers;

pub mod options;
#[doc(inline)]
pub use options::Options;

pub use loggers::*;

#[doc(inline)]
pub use error::Error;
