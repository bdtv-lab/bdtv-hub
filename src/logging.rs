use smaragdine::Printer;
use tracing_subscriber::EnvFilter;

const OWN_TARGET: &str = env!("CARGO_CRATE_NAME");

pub fn init(printer: Printer) {
    let filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::TRACE.into())
        .parse_lossy(&format!("{}=trace", OWN_TARGET));

    tracing_subscriber::fmt()
        // .with_env_filter(filter)
        .with_max_level(tracing::Level::TRACE)
        .with_writer(move || printer.clone())
        .init();
}
