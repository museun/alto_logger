use alto_logger::{
    options::{ColorConfig, StyleConfig, TimeConfig},
    Options,
};

fn main() {
    std::env::set_var("RUST_LOG", "demo=trace");

    let opts = Options::default()
        .with_time(TimeConfig::date_time_format("%c"))
        .with_style(StyleConfig::SingleLine)
        .with_color({
            let mut config = ColorConfig::only_levels();
            config.timestamp = alto_logger::Color::Ansi256(55);
            config
        });

    // make a terminal logger
    let term = alto_logger::TermLogger::new(opts.clone()).unwrap();

    // and a file logger
    let file = alto_logger::FileLogger::timestamp(opts, "out.log").unwrap(); // will make a out-$unix_timestamp.log

    // this will be the path to the file
    let name = file.file_name();

    // combine them so it logs to both
    let logger = alto_logger::MultiLogger::new().with(term).with(file);

    // and then initialize it.
    alto_logger::init(logger).unwrap();

    log::trace!("hello world");
    log::debug!("hello world");
    log::info!("hello world");
    log::warn!("hello world");
    log::error!("hello world");
}
