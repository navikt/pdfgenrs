use log4rs::append::console::ConsoleAppender;
use log4rs::Config;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::json::JsonEncoder;
use log::LevelFilter;

pub fn init_log4rs() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new( JsonEncoder::new()))
        .build();

    let config = Config::builder()
        .appender(Appender::builder()
            .build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("app::logs", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}
