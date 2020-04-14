/*! alto_logger

## filtering
Use the ENV variable `RUST_LOG` with `module_name=level`

`RUST_LOG="tokio=warn,my_module=info,my_module::inner=trace"`

## output
#### single line
```rust
# use alto_logger::*;
alto_logger::init(Options::default().with_style(StyleConfig::SingleLine)).unwrap();
```
```norun
TRACE 0000.003160400s [mio::poll] registering with poller
TRACE 0000.003683900s [mio::sys::windows::selector] register Token(0) Readable | Writable
TRACE 0000.004215900s [mio::sys::windows::selector] set readiness to (empty)
TRACE 0000.004580800s [mio::sys::windows::tcp] scheduling an accept
INFO  0000.004912900s [asta] listening on: 127.0.0.1:6667
TRACE 0000.005196400s [mio::sys::windows::selector] select; timeout=None
TRACE 0000.005473200s [mio::sys::windows::selector] polling IOCP
```

#### multiple lines
```rust
# use alto_logger::*;
alto_logger::init(Options::default().with_style(StyleConfig::MultiLine)).unwrap();
```
```norun
TRACE 0000.004610900s [mio::poll]
⤷ registering with poller
TRACE 0000.005204200s [mio::sys::windows::selector]
⤷ register Token(0) Readable | Writable
TRACE 0000.005798000s [mio::sys::windows::selector]
⤷ set readiness to (empty)
TRACE 0000.006217100s [mio::sys::windows::tcp]
⤷ scheduling an accept
INFO  0000.006675800s [asta]
⤷ listening on: 127.0.0.1:6667
TRACE 0000.007179100s [mio::sys::windows::selector]
⤷ select; timeout=None
TRACE 0000.007552300s [mio::sys::windows::selector]
⤷ polling IOCP
```
*/

#[cfg(all(doctest))]
doc_comment::doctest!("../README.md");

use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write as _;

#[doc(inline)]
pub use termcolor::Color;

use termcolor::{ColorSpec, WriteColor as _};

#[non_exhaustive]
#[derive(Default)]
/// A set of options for configuring the logger
pub struct Options {
    /// The style configuration
    pub style: StyleConfig,
    /// The color configuration
    pub color: ColorConfig,
    /// The time configuration
    pub time: TimeConfig,
}

impl Options {
    /// Use this `StyleConfig` with these `Options`
    pub fn with_style(mut self, style: StyleConfig) -> Self {
        self.style = style;
        self
    }

    /// Use this `ColorConfig` with these `Options`
    pub fn with_color(mut self, color: ColorConfig) -> Self {
        self.color = color;
        self
    }

    /// Use this `TimeConfig` with these `Options`
    pub fn with_time(mut self, time: TimeConfig) -> Self {
        self.time = time;
        self
    }
}

#[non_exhaustive]
/// How the timestamp should be displayed
///
/// Defaults to the Relative timestamp
pub enum TimeConfig {
    /// No timestamp
    None,
    /// Relative timestamp from the start of the program
    Relative,
    #[cfg(feature = "time")]
    /// Timestamp formatted with from UTC 'now'. See [`formatting`](https://docs.rs/time/0.2.9/time/index.html#formatting)
    DateTime(String),
}

impl TimeConfig {
    #[cfg(feature = "time")]
    // Create a DateTime format
    pub fn date_time_format(s: impl ToString) -> Self {
        Self::DateTime(s.to_string())
    }
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self::Relative
    }
}

/// Logger style
///
/// Defaults to MultiLine
#[non_exhaustive]
pub enum StyleConfig {
    /// Use a single-line format
    SingleLine,
    /// Use a multi-line format
    MultiLine,
}

/// Defaults to Multiline
impl Default for StyleConfig {
    fn default() -> Self {
        Self::MultiLine
    }
}

/// Color configuration for the logger
pub struct ColorConfig {
    /// Color for the `TRACE` level. Default: `Blue`
    pub level_trace: Color,
    /// Color for the `DEBUG` level. Default: `Cyan`
    pub level_debug: Color,
    /// Color for the `INFO` level. Default: `Green`
    pub level_info: Color,
    /// Color for the `WARN` level. Default: `Yellow`
    pub level_warn: Color,
    /// Color for the `ERROR` level. Default: `Red`
    pub level_error: Color,

    /// Color for the timestamp field. Default: `#767676`
    pub timestamp: Color,
    /// Color for the target field. Default: `#AF5F5F`
    pub target: Color,
    /// Color for the continuation field. Default: `#3A3A3A`
    pub continuation: Color,
    /// Color for the message field. Default: `#FFFFFF`
    pub message: Color,
}

impl ColorConfig {
    /// Create a monochrome (e.g. all 'white') color configuration
    pub const fn monochrome() -> Self {
        Self {
            level_trace: Color::White,
            level_debug: Color::White,
            level_info: Color::White,
            level_warn: Color::White,
            level_error: Color::White,
            timestamp: Color::White,
            target: Color::White,
            continuation: Color::White,
            message: Color::White,
        }
    }

    /// Only the levels should have the default colors, the rest should be monochrome
    pub const fn only_levels() -> Self {
        Self {
            level_trace: Color::Blue,
            level_debug: Color::Cyan,
            level_info: Color::Green,
            level_warn: Color::Yellow,
            level_error: Color::Red,
            ..Self::monochrome()
        }
    }
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            level_trace: Color::Blue,
            level_debug: Color::Cyan,
            level_info: Color::Green,
            level_warn: Color::Yellow,
            level_error: Color::Red,

            timestamp: Color::Ansi256(243),
            target: Color::Ansi256(131),
            continuation: Color::Ansi256(237),
            message: Color::Ansi256(231),
        }
    }
}

#[derive(Debug)]
/// An error returned by the logger initialization
pub enum Error {
    /// Logger was already set
    SetLogger(log::SetLoggerError),
    #[cfg(feature = "time")]
    /// Invalid time format string
    InvalidFormatString(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SetLogger(err) => write!(f, "{}", err),
            #[cfg(feature = "time")]
            Self::InvalidFormatString(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {
    #[allow(irrefutable_let_patterns)] // this is so we don't have to check every feature
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::SetLogger(err) = &self {
            return Some(err);
        }
        None
    }
}

/// Initialize the logger with the provided options
pub fn init(options: Options) -> Result<(), Error> {
    #[cfg(feature = "time")]
    {
        if let TimeConfig::DateTime(format) = &options.time {
            time::validate_format_string(format).map_err(Error::InvalidFormatString)?;
        }
    }

    let instance = Logger {
        options,
        filters: std::env::var("RUST_LOG")
            .map(|value| {
                let mut mapping = value
                    .split(',')
                    .filter_map(|input| parse(input))
                    .collect::<Vec<_>>();
                match mapping.len() {
                    0 => Filters::Default,
                    d if d < 15 => {
                        mapping.shrink_to_fit();
                        Filters::List(mapping)
                    }
                    _ => Filters::Map(mapping.into_iter().collect()),
                }
            })
            .unwrap_or_default(),
        start: std::time::Instant::now(),
    };

    // enables trace for all
    // TODO this is wrong
    log::set_max_level(log::LevelFilter::Trace);
    log::set_boxed_logger(Box::new(instance)).map_err(Error::SetLogger)
}

struct Logger {
    options: Options,
    filters: Filters,
    start: std::time::Instant,
}

impl Logger {
    fn print(&self, record: &log::Record) {
        let elapsed = self.start.elapsed();

        let Options {
            color,
            time: timestamp,
            style,
            ..
        } = &self.options;

        let buf_writer = termcolor::BufferWriter::stdout(termcolor::ColorChoice::Auto);
        let mut buffer = buf_writer.buffer();

        let level_color = match record.level() {
            log::Level::Error => color.level_error,
            log::Level::Warn => color.level_warn,
            log::Level::Info => color.level_info,
            log::Level::Debug => color.level_debug,
            log::Level::Trace => color.level_trace,
        };

        let _ = buffer.set_color(ColorSpec::new().set_fg(level_color.into()));
        let _ = write!(buffer, "{:<5} ", record.level());
        let _ = buffer.reset();

        match timestamp {
            TimeConfig::None => {}
            TimeConfig::Relative => {
                let _ = buffer.set_color(ColorSpec::new().set_fg(color.timestamp.into()));
                let _ = write!(
                    buffer,
                    "{:04}.{:09}s",
                    elapsed.as_secs(),
                    elapsed.subsec_nanos()
                );
                let _ = buffer.reset();
            }
            #[cfg(feature = "time")]
            TimeConfig::DateTime(format) => {
                let now = time::OffsetDateTime::now().format(&format);
                let _ = buffer.set_color(ColorSpec::new().set_fg(color.timestamp.into()));
                let _ = write!(buffer, "{}", now);
                let _ = buffer.reset();
            }
        }

        let _ = write!(buffer, " [");
        let _ = buffer.set_color(ColorSpec::new().set_fg(color.target.into()));
        let _ = write!(buffer, "{}", record.target());
        let _ = buffer.reset();
        let _ = write!(buffer, "]");

        if let StyleConfig::MultiLine = style {
            let _ = writeln!(buffer);
            let _ = buffer.set_color(ColorSpec::new().set_fg(color.continuation.into()));
            let _ = write!(buffer, "⤷");
            let _ = buffer.reset();
        }

        let _ = buffer.set_color(ColorSpec::new().set_fg(color.message.into()));
        let _ = write!(buffer, " {}", record.args());
        let _ = buffer.reset();
        let _ = writeln!(buffer);

        let _ = buf_writer.print(&buffer);
    }
}

impl log::Log for Logger {
    #[inline(always)]
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        match self.filters.find_module(metadata.target()) {
            Some(level) => metadata.level() <= level,
            None => false,
        }
    }

    #[inline]
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            self.print(record);
        }
    }

    #[inline(always)]
    fn flush(&self) {}
}

#[inline]
fn parse(input: &str) -> Option<(Cow<'static, str>, log::LevelFilter)> {
    #[inline]
    fn level(s: &str) -> Option<log::LevelFilter> {
        macro_rules! eq {
            ($target:expr) => {
                s.eq_ignore_ascii_case($target)
            };
        }

        match () {
            _ if eq!("trace") => log::LevelFilter::Trace,
            _ if eq!("debug") => log::LevelFilter::Debug,
            _ if eq!("info") => log::LevelFilter::Info,
            _ if eq!("warn") => log::LevelFilter::Warn,
            _ if eq!("error") => log::LevelFilter::Error,
            _ => return None,
        }
        .into()
    }

    let mut iter = input.split('=');
    (iter.next()?.to_string().into(), level(iter.next()?)?).into()
}

enum Filters {
    Default,
    List(Vec<(Cow<'static, str>, log::LevelFilter)>),
    Map(HashMap<Cow<'static, str>, log::LevelFilter>),
}

impl Default for Filters {
    fn default() -> Self {
        Self::Default
    }
}

impl Filters {
    #[inline]
    fn find_module(&self, module: &str) -> Option<log::LevelFilter> {
        if let Self::Default = self {
            return None;
        }
        if let Some(level) = self.find_exact(module) {
            return Some(level);
        }

        let mut last = false;
        for (i, ch) in module.char_indices().rev() {
            if last {
                last = false;
                if ch == ':' {
                    if let Some(level) = self.find_exact(&module[..i]) {
                        return Some(level);
                    }
                }
            } else if ch == ':' {
                last = true
            }
        }
        None
    }

    #[inline]
    fn find_exact(&self, module: &str) -> Option<log::LevelFilter> {
        match self {
            Self::Default => None,
            Self::List(levels) => levels
                .iter()
                .find_map(|(m, level)| Some(*level).filter(|_| m == module)),
            Self::Map(levels) => levels.get(module).copied(),
        }
    }
}
