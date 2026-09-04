use std::sync::Arc;

use azalea_chat::FormattedText;
use serde::Deserialize;
use smaragdine::brigadier::{
    builder::literal_argument_builder::literal, command_dispatcher::CommandDispatcher,
    context::CommandContext,
};
use tracing::info;

use crate::{app, console::Src, richtext};

pub(super) fn register(dispatcher: &mut CommandDispatcher<Src>) {
    // 注册 list
    dispatcher.register(
        literal("list")
            .executes_async(|ctx: &CommandContext<Src>| list(Arc::clone(ctx.source.state()))),
    );
}

async fn list(state: Arc<app::State>) -> i32 {
    let player_list = serde_json::json!(richtext::list(state).await);

    let list_string = FormattedText::deserialize(&player_list).unwrap().to_string();

    info!("Player(s) online:\n{}", list_string);

    0
}
