use std::{borrow::Cow, collections::HashMap};

pub(crate) enum Filters {
    Default,
    List(Vec<(Cow<'static, str>, log::LevelFilter)>),
    Map(HashMap<Cow<'static, str>, log::LevelFilter>),
}

impl Default for Filters {
    fn default() -> Self {
        Self::Default
    }
}

impl Filters {
    pub(crate) fn from_env() -> Self {
        std::env::var("RUST_LOG")
            .map(|input| {
                let mut mapping = input.split(',').filter_map(parse).collect::<Vec<_>>();
                match mapping.len() {
                    0 => Filters::Default,
                    d if d < 15 => {
                        mapping.shrink_to_fit();
                        Filters::List(mapping)
                    }
                    _ => Filters::Map(mapping.into_iter().collect()),
                }
            })
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
        if let Self::Default = self {
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
        None
    }

    #[inline]
    pub(crate) fn find_exact(&self, module: &str) -> Option<log::LevelFilter> {
        match self {
            Self::Default => None,
            Self::List(levels) => levels
                .iter()
                .find_map(|(m, level)| Some(*level).filter(|_| m == module)),
            Self::Map(levels) => levels.get(module).copied(),
        }
    }
}

#[inline]
pub(crate) fn parse(input: &str) -> Option<(Cow<'static, str>, log::LevelFilter)> {
    #[inline]
    fn level(s: &str) -> Option<log::LevelFilter> {
        macro_rules! eq {
            ($target:expr) => {
                s.eq_ignore_ascii_case($target)
            };
        }

        match () {
            _ if eq!("trace") => log::LevelFilter::Trace,
            _ if eq!("debug") => log::LevelFilter::Debug,
            _ if eq!("info") => log::LevelFilter::Info,
            _ if eq!("warn") => log::LevelFilter::Warn,
            _ if eq!("error") => log::LevelFilter::Error,
            _ => return None,
        }
        .into()
    }

    let mut iter = input.split('=');
    (iter.next()?.to_string().into(), level(iter.next()?)?).into()
}
