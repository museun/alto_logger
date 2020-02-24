/*! alto_logger

usage:
### single lines
```rust
# use alto_logger::{Style, ColorConfig};
alto_logger::init(Style::SingleLine, ColorConfig::default()).unwrap();
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

### multiple lines
```rust
# use alto_logger::{Style, ColorConfig};
alto_logger::init(Style::MultiLine, ColorConfig::default()).unwrap();
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

use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write as _;

use once_cell::sync::OnceCell;
use termcolor::{Color, ColorSpec, WriteColor as _};

static INSTANCE: OnceCell<Logger> = OnceCell::new();

/// Logger style
#[non_exhaustive]
pub enum Style {
    /// Use a single-line format
    SingleLine,
    /// Use a multi-line format
    MultiLine,
}

/// Defaults to Multiline
impl Default for Style {
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

/// Initialize the logger
pub fn init(style: Style, color: ColorConfig) -> Result<(), log::SetLoggerError> {
    let instance = INSTANCE.get_or_init(|| Logger {
        style,
        color,
        filters: std::env::var("RUST_LOG")
            .map(|value| {
                let mut mapping = value
                    .split(",")
                    .into_iter()
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
    });

    log::set_max_level(log::LevelFilter::Trace);
    log::set_logger(instance)
}

struct Logger {
    style: Style,
    color: ColorConfig,
    filters: Filters,
    start: std::time::Instant,
}

impl Logger {
    fn print(&self, record: &log::Record) {
        let elapsed = self.start.elapsed();

        let buf_writer = termcolor::BufferWriter::stdout(termcolor::ColorChoice::Auto);
        let mut buffer = buf_writer.buffer();

        let color = match record.level() {
            log::Level::Error => self.color.level_error,
            log::Level::Warn => self.color.level_warn,
            log::Level::Info => self.color.level_info,
            log::Level::Debug => self.color.level_debug,
            log::Level::Trace => self.color.level_trace,
        };

        let _ = buffer.set_color(ColorSpec::new().set_fg(color.into()));
        let _ = write!(buffer, "{:<5} ", record.level());
        let _ = buffer.reset();

        let _ = buffer.set_color(ColorSpec::new().set_fg(self.color.timestamp.into()));
        let _ = write!(
            buffer,
            "{:04}.{:09}s",
            elapsed.as_secs(),
            elapsed.subsec_nanos()
        );
        let _ = buffer.reset();

        let _ = write!(buffer, " [");
        let _ = buffer.set_color(ColorSpec::new().set_fg(self.color.target.into()));
        let _ = write!(buffer, "{}", record.target());
        let _ = buffer.reset();
        let _ = write!(buffer, "]");

        match self.style {
            Style::MultiLine => {
                let _ = writeln!(buffer);
                let _ = buffer.set_color(ColorSpec::new().set_fg(self.color.continuation.into()));
                let _ = write!(buffer, "{}", "⤷");
                let _ = buffer.reset();
            }
            _ => {}
        };

        let _ = buffer.set_color(ColorSpec::new().set_fg(self.color.message.into()));
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
