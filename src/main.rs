use bdtv_hub::{App, load_env, logging};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    // 加载 .env 文件到环境变量
    dotenv().ok();

    // 加载环境变量配置
    let config = load_env();

    // 创建应用程序实例
    let app = App::new(config);

    // 初始化日志系统
    logging::init(app.printer());

    // 运行应用程序
    app.run().await;
}
