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

mod color;
mod style;
mod time;

#[doc(inline)]
pub use self::time::TimeConfig;
#[doc(inline)]
pub use color::ColorConfig;
#[doc(inline)]
pub use style::StyleConfig;

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
