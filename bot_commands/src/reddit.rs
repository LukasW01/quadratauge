use bot_core::{
    response::{Response, ResponseBuilder},
    BotCommand, Error,
};
use serenity::{
    async_trait,
    builder::CreateEmbed,
    client::Context,
    model::{application::CommandInteraction, Color},
};

// reddit subreddit json (https://www.reddit.com/r/{subreddit}.json)
// data -> children -> data -> title, selftext, subreddit_name_prefixed, url, created, author
#[derive(Deserialize, Debug)]
struct RedditResponse {
    data: RedditData,
}

#[derive(Deserialize, Debug)]
struct RedditData {
    children: Vec<RedditChild>,
}

#[derive(Deserialize, Debug)]
struct RedditChild {
    data: RedditChildData,
}

#[derive(Deserialize, Debug)]
pub struct RedditChildData {
    title: String,
    selftext: String,
    permalink: String,
    subreddit_name_prefixed: String,
    url: String,
    created: f64,
    author: String,
}

#[derive(CommandBaseline, Default)]
#[description = "Searches the subreddit for your query"]
#[deferred = true]
#[argument(
    name = "subreddit",
    description = "The subreddit you want to search",
    kind = "String"
)]
pub struct Reddit {}

impl Reddit {
    async fn request_reddit_dictionary_entries(
        query: &str,
    ) -> Result<Vec<RedditChildData>, reqwest::Error> {
        debug!("Requesting reddit dictionary and deserialize json body");
        let url = format!("https://www.reddit.com/r/{}.json", query);
        Ok(reqwest::Client::new()
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?
            .json::<RedditResponse>()
            .await?
            .data
            .children
            .into_iter()
            .map(|child| child.data)
            .collect())
    }

    fn create_embed(reddit_entries: Vec<RedditChildData>) -> Vec<CreateEmbed> {
        let mut embeds: Vec<CreateEmbed> = Vec::new();
        for (index, reddit) in reddit_entries.iter().enumerate() {
            if index >= 5 {
                break;
            }
            let mut embed_reddit_entry = CreateEmbed::default()
                .color(Color::from_rgb(255, 0, 0))
                .title(&reddit.title)
                .url(format!("https://www.reddit.com{}", reddit.permalink))
                .field("Subreddit", &reddit.subreddit_name_prefixed, true)
                .field("Author", &reddit.author, true)
                .field(
                    "Created",
                    chrono::DateTime::from_timestamp(reddit.created as i64, 0)
                        .unwrap_or_default()
                        .to_string(),
                    true,
                )
                .field(
                    "Selftext",
                    reddit
                        .selftext
                        .clone()
                        .chars()
                        .take(1024)
                        .collect::<String>(),
                    false,
                );

            if ["jpeg", "png", "jpg", "gif", "webp", "avif"]
                .iter()
                .any(|ext| reddit.url.contains(ext))
            {
                embed_reddit_entry = embed_reddit_entry.image(&reddit.url);
            }

            embeds.push(embed_reddit_entry);
        }
        embeds
    }
}

#[async_trait]
impl BotCommand for Reddit {
    async fn execute<'a>(
        &self,
        _ctx: &Context,
        command: &'a mut CommandInteraction,
        response_builder: &'a mut ResponseBuilder,
    ) -> Result<Response, Error> {
        let query = self.arg_subreddit(command);
        let results = Self::request_reddit_dictionary_entries(&query)
            .await
            .map_err(|err| {
                error!("Failed to request Subreddit entries : {:?}", err);
                Error::Command {
                    message: ":x: *Failed to request Subreddit*".to_string(),
                }
            })?;
        let response_builder = if results.is_empty() {
            response_builder.message(Some(":x: *Nothing found*".to_string()))
        } else {
            response_builder.embeds(Self::create_embed(results))
        };
        Ok(response_builder.build()?)
    }
}
