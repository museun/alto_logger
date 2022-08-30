/// How the timestamp should be displayed
///
/// Several helper methods for constructing this type are provided
/// * [`Relative`](enum.TimeConfig.html#variant.Relative) timestamp style
///     * use [`TimeConfig::relative_now`](enum.TimeConfig.html#method.relative_now) to start the _clock_ from `now`
/// * [`Timing`](enum.TimeConfig.html#variant.Timing) timestamp style
///     * use [`TimeConfig::relative_local`](enum.TimeConfig.html#method.relative_local).
///
/// ***Note*** Defaults to the `None` timestamp
#[derive(Debug)]
#[non_exhaustive]
pub enum TimeConfig {
    /// No timestamp
    None,
    ///
    /// Timestamp since the UNIX epoch
    Unix,
    /// Relative timestamp from the start of the program
    ///
    /// This prints out a fractional number of seconds from when the logger was initialized.
    Relative(std::time::Instant),
    /// Relative timestamp from the previous log statement
    ///
    /// This prints out a fractional number of seconds since the last statement was logged
    Timing(std::sync::Mutex<Option<std::time::Instant>>),

    #[cfg(feature = "time")]
    /// Timestamp formatted with from UTC 'now'. See [`formatting`](https://time-rs.github.io/book/api/format-description.html)
    ///
    /// This allows you to provide a 'fixed' date time. (e.g. UTC offset or unix timestamp or whatever you want)
    DateTime(&'static [time::format_description::FormatItem<'static>]),
}

impl Clone for TimeConfig {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Unix => Self::Unix,
            Self::Relative(inner) => Self::Relative(*inner),
            Self::Timing(_) => Self::Timing(Default::default()),
            #[cfg(feature = "time")]
            Self::DateTime(inner) => Self::DateTime(inner.clone()),
        }
    }
}

impl TimeConfig {
    /// Create a Relative timestamp starting at 'now'
    pub fn relative_now() -> Self {
        Self::Relative(std::time::Instant::now())
    }

    /// Create a Relative timestamp based on the previous logging statement
    pub fn relative_local() -> Self {
        Self::Timing(Default::default())
    }

    /// Create a timestamp based on the UNIX epoch (number of seconds since Jan. 1 1970)
    pub fn unix_timestamp() -> Self {
        Self::Unix
    }

    #[cfg(feature = "time")]
    /// Create a DateTime format
    ///
    /// See the formatting description [here](https://time-rs.github.io/book/api/format-description.html)
    ///
    /// This requires you to use a statically-parsed `format_description`.
    ///
    /// you can get one via [`format_description`](https://docs.rs/time/0.3.14/time/macros/macro.format_description.html)
    /// or using a [well-known format](https://docs.rs/time/0.3.14/time/format_description/well_known/index.html)
    pub fn date_time_format(
        format_description: &'static [time::format_description::FormatItem<'static>],
    ) -> Self {
        Self::DateTime(format_description)
    }
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self::None
    }
}
