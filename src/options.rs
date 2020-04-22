/*! Configuration for various loggers

* [`StyleConfig`](enum.StyleConfig.html) allows you to choose which line-formating you want.
* [`ColorConfig`](struct.ColorConfig.html) allows you to choose colors per element of the terminal logger.
* [`TimeConfig`](enum.TimeConfig.html) allows you to choose which timestamp format to use.

An example:
```rust
# use alto_logger::{Options, options::*};
let opts = Options::default()
    .with_style(StyleConfig::SingleLine)    // use a single-line output
    .with_time(TimeConfig::relative_now())  // use a timestamp relative to 'now'
    .with_color(ColorConfig::only_levels()); // only color the levels
```
*/

use crate::Color;

#[non_exhaustive]
#[derive(Default, Clone, Debug)]
/// Configuration for the logger
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
    pub const fn with_style(mut self, style: StyleConfig) -> Self {
        self.style = style;
        self
    }

    /// Use this `ColorConfig` with these `Options`
    pub const fn with_color(mut self, color: ColorConfig) -> Self {
        self.color = color;
        self
    }

    /// Use this `TimeConfig` with these `Options`
    // NOTE this cannot be const until const dtors are stablized (the 'String' may be dropped)
    pub fn with_time(mut self, time: TimeConfig) -> Self {
        self.time = time;
        self
    }
}

impl From<TimeConfig> for Options {
    fn from(conf: TimeConfig) -> Self {
        Self::default().with_time(conf)
    }
}

impl From<ColorConfig> for Options {
    fn from(conf: ColorConfig) -> Self {
        Self::default().with_color(conf)
    }
}

impl From<StyleConfig> for Options {
    fn from(conf: StyleConfig) -> Self {
        Self::default().with_style(conf)
    }
}

#[non_exhaustive]
/// How the timestamp should be displayed
///
/// ***Note*** Defaults to the `None` timestamp
#[derive(Clone, Debug)]
pub enum TimeConfig {
    /// No timestamp
    None,
    /// Relative timestamp from the start of the program
    Relative(std::time::Instant),
    #[cfg(feature = "time")]
    /// Timestamp formatted with from UTC 'now'. See [`formatting`](https://docs.rs/time/0.2.9/time/index.html#formatting)
    DateTime(String),
}

impl TimeConfig {
    /// Create a Relative timestamp starting at 'now'
    pub fn relative_now() -> Self {
        Self::Relative(std::time::Instant::now())
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

/// Logger line breaking style
///
/// ***Note*** Defaults to MultiLine
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
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
