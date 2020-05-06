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
