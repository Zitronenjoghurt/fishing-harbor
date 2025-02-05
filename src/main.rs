use crate::commands::ping::ping;
use crate::state::AppState;
use poise::serenity_prelude as serenity;

mod commands;
mod enums;
mod state;
mod structures;
mod ui;

type PoiseError = Box<dyn std::error::Error + Send + Sync>;
type PoiseContext<'a> = poise::Context<'a, AppState, PoiseError>;

#[tokio::main]
async fn main() {
    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN is not set");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(AppState {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(&token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
