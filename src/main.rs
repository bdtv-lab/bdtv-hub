mod app;
mod console;
mod envconf;
mod logging;
mod qq;
mod server;
mod signal;
mod types;
mod warden;

use crate::app::{App, AppState};

#[tokio::main]
async fn main() {
    // 创建应用程序实例
    let app = App::new();

    // 初始化日志系统
    logging::init(app.printer());

    // 运行应用程序
    app.run().await;
}
