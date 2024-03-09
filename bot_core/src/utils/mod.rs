use crate::{command::Commands, BotCommand};
use serenity::client::Context;
use std::sync::Arc;

pub mod voice;

pub(crate) async fn get_commands(ctx: &Context) -> Vec<Arc<dyn BotCommand>> {
    let data_read = ctx.data.read().await;
    data_read
        .get::<Commands>()
        .expect("Command array missing")
        .clone()
}
