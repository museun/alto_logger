use crate::{
    filters::Filters,
    options::{Options, StyleConfig, TimeConfig},
};
use std::{io::Write, path::Path, sync::Mutex};

/// `std::io::Write` based logger, intended for `std::fs::File`
pub struct FileLogger<W: Send + 'static> {
    options: Options,
    filters: Filters,
    path: Option<std::path::PathBuf>,
    write: Mutex<W>,
}

impl FileLogger<std::fs::File> {
    /// Create a new file logger that truncates the log file before starting.
    pub fn truncate(
        options: impl Into<Options>,
        path: impl AsRef<Path>,
    ) -> Result<Self, crate::Error> {
        let options = options.into();

        let path = path.as_ref();
        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
            .map(|file| {
                let mut this = Self::new(options, file);
                this.path.replace(path.into());
                this
            })
            .map_err(crate::Error::FileLogger)
    }

    /// Create a new file logger that appends to the log file.
    pub fn append(
        options: impl Into<Options>,
        path: impl AsRef<Path>,
    ) -> Result<Self, crate::Error> {
        let options = options.into();

        let path = path.as_ref();
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(&path)
            .map(|file| {
                let mut this = Self::new(options, file);
                this.path.replace(path.into());
                this
            })
            .map_err(crate::Error::FileLogger)
    }

    /// Create a new file logger with a timestamp appended to its name.
    ///
    /// Example:
    /// * `out.log` will become `out_1587429534.log`
    /// * `out` will become `out_1587429534`
    pub fn timestamp(
        options: impl Into<Options>,
        path: impl AsRef<Path>,
    ) -> Result<Self, crate::Error> {
        let options = options.into();

        fn io_err(reason: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> crate::Error {
            crate::Error::FileLogger(std::io::Error::new(std::io::ErrorKind::Other, reason))
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(io_err)?
            .as_secs();

        // Working with paths is Rust isn't the most fun thing
        let path = path.as_ref();

        let file_stem = path
            .file_stem()
            .ok_or_else(|| io_err("no file stem provided"))?;

        let file_ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        let new_name = file_stem
            .to_str()
            .map(|s| {
                let mut file = s.to_string() + "_" + &now.to_string();
                if !file_ext.is_empty() {
                    file.push('.');
                    file.push_str(file_ext);
                }
                file
            })
            .map(std::path::PathBuf::from)
            .ok_or_else(|| io_err("cannot create new name"))?;

        let path = path.with_file_name(new_name);

        std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)
            .map(|file| {
                let mut this = Self::new(options, file);
                this.path.replace(path);
                this
            })
            .map_err(crate::Error::FileLogger)
    }

    /// Get the path if one was created/provided
    pub fn file_name(&self) -> Option<&Path> {
        self.path.as_deref()
    }
}

impl<W: Write + Send + 'static> FileLogger<W> {
    /// Use this logger as the 'installed' logger (same as alto_logger::init(this);)
    pub fn init(self) -> Result<(), crate::Error> {
        crate::init(self)
    }

    /// Create a new file logger for this writer
    pub fn new(options: impl Into<Options>, writer: W) -> Self {
        let options = options.into();
        Self {
            options,
            filters: Filters::from_env(),
            write: Mutex::new(writer),
            path: None,
        }
    }

    fn print(&self, record: &log::Record<'_>) {
        let Options {
            time: timestamp,
            style,
            ..
        } = &self.options;

        let mut file = self.write.lock().unwrap();
        let _ = write!(file, "{:<5}", record.level());

        match timestamp {
            TimeConfig::None => {}
            TimeConfig::Unix => {
                let elapsed = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("time should not go backwards");
                let _ = write!(file, " {:04}", elapsed.as_secs(),);
            }

            TimeConfig::Relative(start) => {
                let elapsed = start.elapsed();
                let _ = write!(
                    file,
                    " {:04}.{:09}s",
                    elapsed.as_secs(),
                    elapsed.subsec_nanos()
                );
            }

            TimeConfig::Timing(inner) => {
                let inner = &mut *inner.lock().unwrap();
                if let Some(start) = &*inner {
                    let elapsed = start.elapsed();
                    let _ = write!(
                        file,
                        " {:04}.{:09}s",
                        elapsed.as_secs(),
                        elapsed.subsec_nanos()
                    );
                } else {
                    let _ = write!(file, " {:04}.{:09}s", 0, 0);
                }
                inner.replace(std::time::Instant::now());
            }

            #[cfg(feature = "time")]
            TimeConfig::DateTime(format) => {
                let now = time::OffsetDateTime::now().format(&format);
                let _ = write!(file, " {}", now);
            }
        }

        let _ = write!(file, " [");
        let _ = write!(file, "{}", record.target());
        let _ = write!(file, "]");

        if let StyleConfig::MultiLine = style {
            let _ = writeln!(file);
            let _ = write!(file, "⤷");
        }

        let _ = write!(file, " {}", record.args());
        let _ = writeln!(file);
    }
}

impl<W: Write + Send + 'static> log::Log for FileLogger<W> {
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
    fn flush(&self) {
        let _ = self.write.lock().unwrap().flush();
    }
}
