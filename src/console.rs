use std::sync::Arc;

use smaragdine::{Console, Exit, Source, brigadier::prelude::*};
use tokio_util::sync::CancellationToken;
use tracing::error;

use crate::AppState;

pub async fn console(state: Arc<AppState>, token: CancellationToken) {
    let console = Console::builder()
        .printer(state.printer.clone())
        .prompt("/")
        .multiline_prompt("/")
        .completion_prompt("/")
        // 以 Minecraft 风格注册命令
        .command(literal("ping").executes(|ctx: &CommandContext<Source<_>>| {
            ctx.source.printer().print("pong!");
            1
        }))
        .build(state);

    let source = console.source();
    let mut task = tokio::task::spawn_blocking(move || console.run());

    tokio::select! {
        result = &mut task => match result {
            // 正常退出
            Ok(Exit::Quit(()) | Exit::Interrupted) => token.cancel(),
            // 无可用终端
            Ok(Exit::NoTerminal) => {}
            // 一般错误
            Ok(Exit::Failed(error)) => error!("console error: {error}"),
            // 致命错误
            Err(error) => {
                error!("console task failed: {error}");
                token.cancel();
            }
        },
        _ = token.cancelled() => {
            source.request_quit();
            if let Err(error) = task.await {
                error!("console task failed: {error}");
            }
        }
    }
}
