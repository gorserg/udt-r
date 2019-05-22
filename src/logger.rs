use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::{
    policy::compound::roll::fixed_window::FixedWindowRoller,
    policy::compound::trigger::size::SizeTrigger, policy::compound::CompoundPolicy,
    RollingFileAppender,
};
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn init(log_file: &str) {
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} - {m}{n}")))
        .build();

    let roller = FixedWindowRoller::builder()
        .build(&format!("log/backup/{}.{{}}.gz", "log"), 32)
        .unwrap();
    let trigger = SizeTrigger::new(1024 * 1024); // 1 Mb
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

    let file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} - {m}{n}")))
        .build(format!("log/{}", log_file), Box::new(policy))
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appender("file")
                .appender("console")
                .build(LevelFilter::Info),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}
