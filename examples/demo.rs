use alto_logger::{ColorConfig, Options, StyleConfig, TimeConfig};

fn main() {
    std::env::set_var("RUST_LOG", "demo=trace");

    alto_logger::init(
        Options::default()
            .with_time(TimeConfig::date_time_format("%c"))
            .with_style(StyleConfig::SingleLine)
            .with_color({
                let mut config = ColorConfig::only_levels();
                config.timestamp = alto_logger::Color::Ansi256(55);
                config
            }),
    )
    .unwrap();

    log::trace!("hello world");
    log::debug!("hello world");
    log::info!("hello world");
    log::warn!("hello world");
    log::error!("hello world");
}
