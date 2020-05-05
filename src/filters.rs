use std::{borrow::Cow, collections::HashMap};

#[derive(Debug)]
pub(crate) enum FiltersKind {
    Default,
    List(Vec<(Cow<'static, str>, log::LevelFilter)>),
    Map(HashMap<Cow<'static, str>, log::LevelFilter>),
}

#[derive(Debug)]
pub(crate) struct Filters {
    kind: FiltersKind,
    minimum: Option<log::LevelFilter>,
}

impl Default for Filters {
    fn default() -> Self {
        Self {
            kind: FiltersKind::Default,
            minimum: None,
        }
    }
}

impl Filters {
    pub(crate) fn from_str(input: &str) -> Self {
        let mut mapping = input.split(',').filter_map(parse).collect::<Vec<_>>();

        let minimum = input
            .split(',')
            .filter(|s| !s.contains('='))
            .flat_map(|s| s.parse().ok())
            .filter(|&l| l != log::LevelFilter::Off)
            .max();

        let kind = match mapping.len() {
            0 => FiltersKind::Default,
            d if d < 15 => {
                mapping.shrink_to_fit();
                FiltersKind::List(mapping)
            }
            _ => FiltersKind::Map(mapping.into_iter().collect()),
        };

        Self { kind, minimum }
    }

    pub(crate) fn from_env() -> Self {
        std::env::var("RUST_LOG")
            .map(|s| Self::from_str(&s))
            .unwrap_or_default()
    }

    #[inline]
    pub(crate) fn is_enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        match self.find_module(metadata.target()) {
            Some(level) => metadata.level() <= level,
            None => false,
        }
    }

    #[inline]
    pub(crate) fn find_module(&self, module: &str) -> Option<log::LevelFilter> {
        if let FiltersKind::Default = self.kind {
            return None;
        }

        if let Some(level) = self.find_exact(module) {
            return Some(level);
        }

        let mut last = false;
        for (i, ch) in module.char_indices().rev() {
            if last {
                last = false;
                if ch == ':' {
                    if let Some(level) = self.find_exact(&module[..i]) {
                        return Some(level);
                    }
                }
            } else if ch == ':' {
                last = true
            }
        }

        self.minimum
    }

    #[inline]
    pub(crate) fn find_exact(&self, module: &str) -> Option<log::LevelFilter> {
        match &self.kind {
            FiltersKind::Default => None,
            FiltersKind::List(levels) => levels
                .iter()
                .find_map(|(m, level)| Some(*level).filter(|_| m == module)),
            FiltersKind::Map(levels) => levels.get(module).copied(),
        }
    }
}

#[inline]
pub(crate) fn parse(input: &str) -> Option<(Cow<'static, str>, log::LevelFilter)> {
    let mut iter = input.split('=');
    Some((
        Cow::Owned(iter.next()?.to_string()),
        iter.next()?.to_ascii_uppercase().parse().ok()?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn filters() {
        let input = "debug,foo::bar=off,foo::baz=trace,foo=info,baz=off,quux=error";
        let filters = Filters::from_str(input);

        let modules = &[
            ("foo::bar", log::LevelFilter::Off),
            ("foo::baz", log::LevelFilter::Trace),
            ("foo", log::LevelFilter::Info),
            ("baz", log::LevelFilter::Off),
            ("quux", log::LevelFilter::Error),
            ("something", log::LevelFilter::Debug),
            ("another::thing", log::LevelFilter::Debug),
        ];

        for (module, expected) in modules {
            assert_eq!(filters.find_module(module).unwrap(), *expected);
        }
    }
}
