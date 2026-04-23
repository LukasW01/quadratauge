use bot_core::{
    BotCommand, Error,
    response::{Response, ResponseBuilder},
    utils::{self},
};
use serenity::{
    async_trait,
    builder::CreateEmbed,
    client::Context,
    model::{Color, application::CommandInteraction},
};
use songbird::{input::AuxMetadata, tracks::LoopState};

#[derive(CommandBaseline, Default)]
#[description = "List all tracks in the queue"]
#[deferred = true]
pub struct Tracks {}

#[async_trait]
impl BotCommand for Tracks {
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
            response_builder.message(Some(":x: **No tracks in the queue**".to_string()))
        } else {
            let queue_snapshot = handler.queue().current_queue();
            let mut embeded_tracks = CreateEmbed::default()
                .color(Color::BLURPLE)
                .title("Track List");
            for (index, track) in queue_snapshot.into_iter().enumerate() {
                let track_position = index + 1;
                // Extract title and url of the track. This is scoped to drop the read lock on
                // the track meta as soon as possible.
                let (title, url, loop_state) = {
                    // Extract track Metadata from tracks TyeMap
                    let metadata = track.data::<AuxMetadata>();
                    let title = metadata
                        .title
                        .as_ref()
                        .map_or("**No title provided**", |t| t);
                    let url = metadata
                        .source_url
                        .as_ref()
                        .map_or("**No url provided**", |u| u);

                    // Extract loop state from track state
                    let track_info = track.get_info().await.unwrap();
                    (title.to_owned(), url.to_owned(), track_info.loops)
                };
                let mut embed_value = format!(":notes: `{url}`");
                match loop_state {
                    LoopState::Infinite => {
                        embed_value.push_str("\n:repeat: `Infinite`");
                    }
                    LoopState::Finite(loop_amount) => {
                        if loop_amount.get() > 0 {
                            embed_value.push_str(&format!("\n:repeat: `{}`", loop_amount.get()));
                        }
                    }
                }
                embeded_tracks = embeded_tracks.field(
                    format!("{track_position}. :newspaper: `{title}`"),
                    embed_value,
                    false,
                );
            }
            response_builder.embeds(vec![embeded_tracks])
        };
        Ok(response_builder.build()?)
    }
}
