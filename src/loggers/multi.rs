use crate::filters::Filters;

/// A multi-logger
///
/// This allows for multiple loggers to be used
pub struct MultiLogger {
    filters: Filters,
    loggers: Vec<Box<dyn log::Log>>,
}

impl MultiLogger {
    /// Use this logger as the 'installed' logger (same as alto_logger::init(this);)
    pub fn init(self) -> Result<(), crate::Error> {
        crate::init(self)
    }

    /// Create a new Multilogger without any loggers
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            filters: Filters::from_env(),
            loggers: Vec::new(),
        }
    }

    /// Add a logger to this multilogger
    pub fn with(mut self, logger: impl log::Log + 'static) -> Self {
        self.loggers.push(Box::new(logger));
        self
    }
}

impl log::Log for MultiLogger {
    #[inline]
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        self.filters.is_enabled(metadata)
    }

    #[inline]
    fn log(&self, record: &log::Record<'_>) {
        for logger in &self.loggers {
            logger.log(record);
        }
    }

    #[inline]
    fn flush(&self) {
        for logger in &self.loggers {
            logger.flush();
        }
    }
}
