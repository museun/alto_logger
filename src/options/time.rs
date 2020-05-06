#[non_exhaustive]
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
pub enum TimeConfig {
    /// No timestamp
    None,
    /// Relative timestamp from the start of the program
    ///
    /// This prints out a fractional number of seconds from when the logger was initialized.
    Relative(std::time::Instant),
    /// Relative timestamp from the previous log statement
    ///
    /// This prints out a fractional number of seconds since the last statement was logged        
    Timing(std::sync::Mutex<Option<std::time::Instant>>),
    #[cfg(feature = "time")]
    /// Timestamp formatted with from UTC 'now'. See [`formatting`](https://docs.rs/time/0.2.9/time/index.html#formatting)
    ///
    /// This allows you to provide a 'fixed' date time. (e.g. UTC offset or unix timestamp or whatever you want)    
    DateTime(String),
}

impl Clone for TimeConfig {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
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

    #[cfg(feature = "time")]
    /// Create a DateTime format
    pub fn date_time_format(s: impl ToString) -> Self {
        Self::DateTime(s.to_string())
    }
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self::None
    }
}
