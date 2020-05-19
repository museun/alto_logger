use crate::{
    filters::Filters,
    options::{Options, StyleConfig, TimeConfig},
};
use termcolor::ColorSpec;

/// Stdout logger which supports colors
///
/// If 'NO_COLOR' env var is set, it'll override and disable any color configurations.
pub struct TermLogger {
    options: Options,
    filters: Filters,
    color_choice: termcolor::ColorChoice,
}

impl Default for TermLogger {
    fn default() -> Self {
        Self {
            options: Options::default(),
            filters: Filters::from_env(),
            color_choice: determine_color_choice(),
        }
    }
}

impl TermLogger {
    /// Use this logger as the 'installed' logger (same as `alto_logger::init(this);`)
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
            color_choice: determine_color_choice(),
        })
    }

    fn print(&self, record: &log::Record<'_>) {
        let buf_writer = termcolor::BufferWriter::stdout(self.color_choice);
        let mut buffer = buf_writer.buffer();

        self.render_level(&record, &mut buffer);
        self.render_timestamp(&record, &mut buffer);
        self.render_target(&record, &mut buffer);
        self.render_payload(&record, &mut buffer);

        let _ = buf_writer.print(&buffer);
    }

    fn render_level(
        &self,
        record: &log::Record<'_>,
        buffer: &mut (impl std::io::Write + termcolor::WriteColor),
    ) {
        let color = &self.options.color;

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
    }

    fn render_timestamp(
        &self,
        _record: &log::Record<'_>,
        buffer: &mut (impl std::io::Write + termcolor::WriteColor),
    ) {
        let Options { color, time, .. } = &self.options;

        match time {
            TimeConfig::None => {}

            TimeConfig::Unix => {
                let elapsed = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("time should not go backwards");
                let _ = buffer.set_color(ColorSpec::new().set_fg(color.timestamp.into()));
                let _ = write!(buffer, " {:04}s", elapsed.as_secs(),);
                let _ = buffer.reset();
            }

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
                let _ = write!(buffer, " {}", now);
                let _ = buffer.reset();
            }
        }
    }

    fn render_target(
        &self,
        record: &log::Record<'_>,
        buffer: &mut (impl std::io::Write + termcolor::WriteColor),
    ) {
        let color = &self.options.color;

        let _ = write!(buffer, " [");
        let _ = buffer.set_color(ColorSpec::new().set_fg(color.target.into()));
        let _ = write!(buffer, "{}", record.target());
        let _ = buffer.reset();
        let _ = write!(buffer, "]");
    }

    fn render_payload(
        &self,
        record: &log::Record<'_>,
        buffer: &mut (impl std::io::Write + termcolor::WriteColor),
    ) {
        let Options { style, color, .. } = &self.options;

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

fn determine_color_choice() -> termcolor::ColorChoice {
    if std::env::var("NO_COLOR").is_ok() {
        termcolor::ColorChoice::Never
    } else {
        termcolor::ColorChoice::Auto
    }
}
