use log::{self, Level, LevelFilter, Log, Metadata, Record};
use crate::arch::cpu::cpu_id;
use crate::console::print_in_color;
struct SimpleLogger;
impl Log for SimpleLogger{
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level()  <= log::max_level()
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            print_in_color(
                format_args!(
                    "[{:>5}] [{},-] {}\n",
                    record.level(),
                    cpu_id(),
                    record.args()
                ),
                level_to_color_code(record.level()),
            );
        }
    }
    fn flush(&self) {}
}
static LOGGER: SimpleLogger = SimpleLogger;
pub fn init(){
    log::set_logger(&LOGGER).unwrap();
	log::set_max_level(match option_env!("LOG") {
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}


fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}