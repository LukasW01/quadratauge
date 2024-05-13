use bot_core::{
    response::{Response, ResponseBuilder},
    utils, BotCommand, Error,
};
use serenity::{async_trait, client::Context, model::application::CommandInteraction};

#[derive(CommandBaseline, Default)]
#[description = "Skip current song"]
#[deferred = true]
pub struct Skip {}

#[async_trait]
impl BotCommand for Skip {
    async fn execute<'a>(
        &self,
        ctx: &Context,
        command: &'a mut CommandInteraction,
        response_builder: &'a mut ResponseBuilder,
    ) -> Result<Response, Error> {
        let guild_id = command.guild_id.ok_or(Error::Command {
            message: ":x: **This command can only be executed on a server**".to_string(),
        })?;
        let manager = utils::voice::get_songbird(ctx).await;
        let call = manager.get(guild_id).ok_or(Error::Command {
            message: ":x: **No active voice session on the server**".to_string(),
        })?;
        let handler = call.lock().await;
        let response_builder = if handler.queue().is_empty() {
            response_builder.message(Some(":x: **Nothing to skip**".to_string()))
        } else {
            handler.queue().skip().map_err(|err| {
                error!("Failed to skip: {err:?}");
                Error::Command {
                    message: ":x: **Could not skip the track**".to_string(),
                }
            })?;
            response_builder.message(Some(":fast_forward: **Skipped current song**".to_string()))
        };
        Ok(response_builder.build()?)
    }
}
