use std::sync::Arc;

use smaragdine::Printer;
use tokio::{
    sync::{Mutex, mpsc},
    task::JoinSet,
};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::{console::console, server::http_server, signal::shutdown_signal, warden::warden};

pub enum AppEvent {
    PlayerJoined,
    PlayerLeft,
}

#[derive(Debug)]
pub struct AppState {
    pub online_players: Mutex<u32>,
    pub printer: Printer,
    pub event_tx: mpsc::Sender<AppEvent>,
}

impl AppState {
    pub fn new(tx: mpsc::Sender<AppEvent>) -> Self {
        Self {
            online_players: Mutex::new(0),
            printer: Printer::new(),
            event_tx: tx,
        }
    }
}

/// 应用程序的主结构体
pub struct App {
    state: Arc<AppState>,
    token: CancellationToken,
}

impl App {
    pub fn new() -> Self {
        // 初始化 QQ 通信通道
        let (tx, rx) = mpsc::channel(100);

        // 初始化应用状态
        let state = Arc::new(AppState::new(tx));

        // 创建取消 token
        let token = CancellationToken::new();

        Self { state, token }
    }

    pub async fn run(self) {
        // 创建任务集合
        let mut tasks = JoinSet::new();

        // 启动控制台
        tasks.spawn(console(self.state(), self.token()));
        // 启动 http 服务器
        tasks.spawn(http_server(self.state(), self.token()));
        // 启动在线状态巡检
        tasks.spawn(warden(self.state(), self.token()));
        // 启动关闭信号监听
        tasks.spawn(shutdown_signal(self.token()));

        // 阻塞等待任务事件
        while let Some(res) = tasks.join_next().await {
            if let Err(e) = res {
                error!("发生 panic: {e}");
            }
        }
        info!("服务已退出");
    }

    /// 安全获取一份 AppState 的引用
    fn state(&self) -> Arc<AppState> {
        Arc::clone(&self.state)
    }

    /// 安全获取一份 CancellationToken 的引用
    fn token(&self) -> CancellationToken {
        self.token.clone()
    }

    /// 安全获取一份 Printer 的引用
    pub fn printer(&self) -> Printer {
        self.state.printer.clone()
    }
}
