use std::io::Write as _;

use crate::{
    filters::Filters,
    options::{Options, StyleConfig, TimeConfig},
};
use termcolor::{ColorSpec, WriteColor as _};

/// Stdout logger which supports colors
///
/// If 'NO_COLOR' env var is set, it'll override any color configurations.
pub struct TermLogger {
    options: Options,
    filters: Filters,

    // TODO mention that it'll honor NO_COLOR
    disable_colors: bool,
}

impl Default for TermLogger {
    fn default() -> Self {
        Self {
            options: Options::default(),
            filters: Filters::from_env(),
            disable_colors: std::env::var("NO_COLOR").is_ok(),
        }
    }
}

impl TermLogger {
    /// Use this logger as the 'installed' logger (same as alto_logger::init(this);)
    pub fn init(self) -> Result<(), crate::Error> {
        crate::init(self)
    }

    /// Create a new terminal logger
    pub fn new(options: impl Into<Options>) -> Result<Self, crate::Error> {
        let options = options.into();

        #[cfg(feature = "time")]
        {
            if let TimeConfig::DateTime(format) = &options.time {
                time::validate_format_string(format).map_err(crate::Error::InvalidFormatString)?;
            }
        }

        Ok(Self {
            options,
            filters: Filters::from_env(),
            disable_colors: std::env::var("NO_COLOR").is_ok(),
        })
    }

    fn print(&self, record: &log::Record<'_>) {
        let Options {
            color,
            time: timestamp,
            style,
            ..
        } = &self.options;

        let buf_writer = termcolor::BufferWriter::stdout(if self.disable_colors {
            termcolor::ColorChoice::Never
        } else {
            termcolor::ColorChoice::Auto
        });
        let mut buffer = buf_writer.buffer();

        let level_color = match record.level() {
            log::Level::Error => color.level_error,
            log::Level::Warn => color.level_warn,
            log::Level::Info => color.level_info,
            log::Level::Debug => color.level_debug,
            log::Level::Trace => color.level_trace,
        };

        let _ = buffer.set_color(ColorSpec::new().set_fg(level_color.into()));
        let _ = write!(buffer, "{:<5}", record.level());
        let _ = buffer.reset();

        match timestamp {
            TimeConfig::None => {}
            TimeConfig::Relative(start) => {
                let elapsed = start.elapsed();
                let _ = buffer.set_color(ColorSpec::new().set_fg(color.timestamp.into()));
                let _ = write!(
                    buffer,
                    " {:04}.{:09}s",
                    elapsed.as_secs(),
                    elapsed.subsec_nanos()
                );
                let _ = buffer.reset();
            }

            TimeConfig::Timing(inner) => {
                let inner = &mut *inner.lock().unwrap();
                if let Some(start) = &*inner {
                    let elapsed = start.elapsed();
                    let _ = buffer.set_color(ColorSpec::new().set_fg(color.timestamp.into()));
                    let _ = write!(
                        buffer,
                        " {:04}.{:09}s",
                        elapsed.as_secs(),
                        elapsed.subsec_nanos()
                    );
                    let _ = buffer.reset();
                }
                inner.replace(std::time::Instant::now());
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

impl log::Log for TermLogger {
    #[inline]
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        self.filters.is_enabled(metadata)
    }

    #[inline]
    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            self.print(record);
        }
    }

    #[inline]
    fn flush(&self) {}
}
