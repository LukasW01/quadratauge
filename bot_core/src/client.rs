use crate::{
    command::Commands, error::Error, handler::command::Handler, http::HttpClientKey,
    intents::botIntents, botCommand,
};
use serenity::{client::Client, model::gateway::GatewayIntents};
use songbird::SerenityInit;
use std::sync::Arc;

#[derive(derive_builder::Builder)]
pub struct bot {
    token: String,
    #[builder(default)]
    commands: Vec<Arc<dyn botCommand>>,
    #[builder(default = "botIntents::default().into()")]
    intents: GatewayIntents,
}

impl bot {
    #[must_use]
    pub fn builder() -> botBuilder {
        botBuilder::default()
    }

    /// This will actually start the configured bot bot
    pub async fn start(self) -> Result<(), Error> {
        let mut client = Client::builder(self.token, self.intents)
            .event_handler(Handler)
            .register_songbird()
            .type_map_insert::<Commands>(self.commands)
            .type_map_insert::<HttpClientKey>(reqwest::Client::new())
            .await
            .map_err(|err| Error::Start { source: err })?;
        client
            .start()
            .await
            .map_err(|err| Error::Start { source: err })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fail_to_build_without_token() {
        let build = bot::builder().commands(vec![]).build();
        assert!(build.is_err())
    }

    #[test]
    fn build_with_token() {
        let build = bot::builder()
            .commands(vec![])
            .token("some-token".to_string())
            .build();
        assert!(build.is_ok())
    }

    #[test]
    fn build_with_default_intents() {
        let build = bot::builder()
            .commands(vec![])
            .token("some-token".to_string())
            .build()
            .unwrap();
        assert_eq!(build.intents, botIntents::default().into());
    }

    #[test]
    fn build_with_empty_commands() {
        let build = bot::builder()
            .commands(vec![])
            .token("some-token".to_string())
            .build()
            .unwrap();
        assert!(build.commands.is_empty());
    }
}
