use crate::commands::ping::ping;
use crate::state::AppState;
use fish_lib::config::{Config, ConfigBuilderInterface};
use poise::serenity_prelude as serenity;

mod commands;
mod database;
mod enums;
mod errors;
mod models;
mod repositories;
mod schema;
mod service_provider;
mod services;
mod state;
mod structures;
#[cfg(test)]
mod tests;
mod ui;

type PoiseError = Box<dyn std::error::Error + Send + Sync>;
type PoiseContext<'a> = poise::Context<'a, AppState, PoiseError>;

#[tokio::main]
async fn main() {
    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN is not set");
    let intents = serenity::GatewayIntents::non_privileged();

    let db_url_bot = std::env::var("DATABASE_URL_BOT").expect("DATABASE_URL_BOT is not set");
    let db_url_game = std::env::var("DATABASE_URL_GAME").expect("DATABASE_URL_GAME is not set");
    let config = Config::builder().build();
    let app_state = AppState::new(&db_url_bot, &db_url_game, config);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(app_state)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(&token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
