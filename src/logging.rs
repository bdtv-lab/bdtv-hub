use smaragdine::Printer;

pub fn init(printer: Printer) {
    tracing_subscriber::fmt()
        .with_writer(move || printer.clone())
        .init();
}
