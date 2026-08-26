use smaragdine::Printer;
use time::UtcOffset;
use time::macros::format_description;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::OffsetTime;

const OWN_TARGET: &str = env!("CARGO_CRATE_NAME");

pub fn init(printer: Printer) {
    let filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::WARN.into())
        .parse_lossy(format!("{OWN_TARGET}=trace"));

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
