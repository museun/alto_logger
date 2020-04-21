#[derive(Debug)]
/// An error returned by the logger initialization
pub enum Error {
    /// Logger was already set
    SetLogger(log::SetLoggerError),
    /// An i/o error occured when opening a file logger
    FileLogger(std::io::Error),
    #[cfg(feature = "time")]
    /// Invalid time format string
    InvalidFormatString(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SetLogger(err) => write!(f, "{}", err),
            Self::FileLogger(err) => write!(f, "{}", err),
            #[cfg(feature = "time")]
            Self::InvalidFormatString(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SetLogger(err) => Some(err),
            Self::FileLogger(err) => Some(err),
            #[cfg(feature = "time")]
            _ => None,
        }
    }
}
