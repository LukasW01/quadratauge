use bot_core::{
    BotCommand, Error,
    response::{Response, ResponseBuilder},
    utils,
};
use serenity::{async_trait, client::Context, model::application::CommandInteraction};

#[derive(CommandBaseline, Default)]
#[description = "Pause the current song"]
#[deferred = true]
pub struct Pause {}

#[async_trait]
impl BotCommand for Pause {
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
        if handler.queue().is_empty() {
            response_builder.message(Some(":x: **Nothing to pause**".to_string()));
        } else {
            handler.queue().pause().map_err(|err| {
                error!("Failed to pause: {err:?}");
                Error::Command {
                    message: ":x: **Could not pause the track**".to_string(),
                }
            })?;
            response_builder.message(Some(":pause_button: **Paused**".to_string()));
        }
        Ok(response_builder.build()?)
    }
}
