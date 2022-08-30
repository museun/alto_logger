/*!
## filtering
Use the ENV variable `RUST_LOG` with `module_name=level`

`RUST_LOG="tokio=warn,my_module=info,my_module::inner=trace"`

A default level can be provided with just ***level***. e.g. `RUST_LOG=trace` will enable `trace` for all modules.

You can disable specific modules/crates by using the `off` level

## optional features
* `time` allows formatting a UTC timestamp with the [`time`](time) crate.
    * see the formatting description [here](https://time-rs.github.io/book/api/format-description.html)
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
///
/// And using the shorthands
/// ```rust,ignore
/// use alto_logger::*;
///
/// // statically parse a format description
/// let fmt = time::macros::format_description!("[hour]:[minute]:[second]");
///
/// MultiLogger::new()
///     .with(TermLogger::default())
///      // date_time_format requires the `time` feature
///     .with(FileLogger::append(TimeConfig::date_time_format(format), "output.log").unwrap())
///     .init()
///     .expect("init logger");
/// ```
///
pub fn init(logger: impl log::Log + 'static) -> Result<(), Error> {
    // enables trace for all
    log::set_max_level(log::LevelFilter::Trace);
    log::set_boxed_logger(Box::new(logger)).map_err(Error::SetLogger)
}

/// Convenience function to create a default terminal logger
///
/// This defaults to using:
/// * no timestamp
/// * default colors
/// * multi-line output
pub fn init_term_logger() -> Result<(), Error> {
    TermLogger::new(Options::default()).and_then(init)
}

/// Convenience function to create a terminal logger that uses a single-line output, and unix timestamps.
pub fn init_alt_term_logger() -> Result<(), Error> {
    TermLogger::new(
        Options::default()
            .with_style(StyleConfig::SingleLine)
            .with_time(TimeConfig::unix_timestamp()),
    )
    .and_then(init)
}

mod error;
mod filters;
mod loggers;

pub mod options;
#[doc(inline)]
pub use options::*;

pub use loggers::*;

#[doc(inline)]
pub use error::Error;
