use serenity::model::gateway::GatewayIntents;

pub(crate) struct BotIntents {
    inner: GatewayIntents,
}

impl BotIntents {
    pub fn default() -> Self {
        Self {
            inner: GatewayIntents::empty()
                | GatewayIntents::GUILD_VOICE_STATES
                | GatewayIntents::GUILDS,
        }
    }

    pub fn inner(&self) -> GatewayIntents {
        self.inner
    }
}

impl From<BotIntents> for GatewayIntents {
    fn from(c_intents: BotIntents) -> Self {
        c_intents.inner()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_have_voice_intent() {
        let c_intents: GatewayIntents = BotIntents::default().into();
        assert!(c_intents.guild_voice_states());
    }

    #[test]
    fn should_have_guild_intents() {
        let c_intents: GatewayIntents = BotIntents::default().into();
        assert!(c_intents.guilds());
    }
}
