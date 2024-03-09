use bot_core::{
    response::{Response, ResponseBuilder},
    BotCommand, Error,
};
use serenity::{async_trait, client::Context, model::application::CommandInteraction};

#[derive(CommandBaseline, Default)]
#[description = "Play Ping-Pong"]
pub struct Ping {}

#[async_trait]
impl BotCommand for Ping {
    async fn execute<'a>(
        &self,
        _ctx: &Context,
        _command: &'a mut CommandInteraction,
        response_builder: &'a mut ResponseBuilder,
    ) -> Result<Response, Error> {
        Ok(response_builder
            .message(Some("Pong!".to_string()))
            .build()?)
    }
}
