use log::Level;
use log4rs::{
    append::console::ConsoleAppender,
    append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller,
    append::rolling_file::policy::compound::trigger::size::SizeTrigger,
    append::rolling_file::policy::compound::CompoundPolicy,
    append::rolling_file::RollingFileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

use ansi_term::Colour;
use log::LevelFilter;

struct ColorEncoder;
impl std::fmt::Debug for ColorEncoder {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("ColorEncoder")
    }
}

impl log4rs::encode::Encode for ColorEncoder {
    fn encode(
        &self,
        buf: &mut dyn log4rs::encode::Write,
        record: &log::Record,
    ) -> Result<(), anyhow::Error> {
        let colored_message = match record.level() {
            Level::Info => Colour::Green.paint(format!(
                "INFO - {} - {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.args()
            )),
            Level::Error => Colour::Red.paint(format!(
                "ERROR - {} - {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.args()
            )),
            Level::Debug => Colour::Blue.paint(format!(
                "DEBUG - {} - {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.args()
            )),
            Level::Warn => Colour::Yellow.paint(format!(
                "WARN - {} - {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.args()
            )),
            Level::Trace => Colour::Purple.paint(format!(
                "TRACE - {} - {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.args()
            )),
        };

        writeln!(buf, "{}", colored_message).map_err(|e| anyhow::Error::new(e))
    }
}

pub fn init_logger() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{l} - {d(%Y-%m-%d %H:%M:%S)} - {m}{n}",
        )))
        .encoder(Box::new(ColorEncoder))
        .build();

    let size_trigger = SizeTrigger::new(1024 * 10); // 1MB
    let window_roller = FixedWindowRoller::builder()
        .base(1)
        .build("logs/test.{}.log", 30)
        .unwrap();

    let compound_policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(window_roller));
    let file_appender = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{l} - {d(%Y-%m-%d %H:%M:%S)} - {m}{n}",
        )))
        .build("logs/log.log", Box::new(compound_policy))
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(LevelFilter::Info),
        )
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();
}
