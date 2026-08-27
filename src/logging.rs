use std::env;

use smaragdine::Printer;
use time::UtcOffset;
use time::macros::format_description;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::OffsetTime;

const OWN_TARGET: &str = env!("CARGO_CRATE_NAME");
const LOG_LEVEL_KEY: &str = "LOG_LEVEL";
const DEFAULT_LOG_LEVEL: &str = "info";

pub fn init(printer: Printer) {
    let level = env::var(LOG_LEVEL_KEY)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_LOG_LEVEL.to_string());

    let filter = EnvFilter::builder().parse_lossy(format!("warn,{OWN_TARGET}={level}"));

    let offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    let timer = OffsetTime::new(
        offset,
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    );

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_timer(timer)
        .with_writer(move || printer.clone())
        .init();
}
