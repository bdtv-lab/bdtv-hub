use crate::AppState;

pub fn init(state: &AppState) {
    let printer = state.printer.clone();

    tracing_subscriber::fmt()
        .with_writer(move || printer.clone())
        .init();
}
