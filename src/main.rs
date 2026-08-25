use bdtv_hub::{App, load_env, logging};

#[tokio::main]
async fn main() {
    // 加载环境变量配置
    let config = load_env();

    // 创建应用程序实例
    let app = App::new(config);

    // 初始化日志系统
    logging::init(app.printer());

    // 运行应用程序
    app.run().await;
}
