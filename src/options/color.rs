use crate::Color;

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
