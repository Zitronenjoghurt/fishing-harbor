use crate::enums::colors::Colors;
use crate::errors::command::CommandError;
use crate::service_provider::ServiceProviderInterface;
use crate::ui::embeds::create_user_color_embed;
use crate::{PoiseContext, PoiseError};
use poise::CreateReply;

#[poise::command(slash_command)]
pub async fn ping(ctx: PoiseContext<'_>) -> Result<(), PoiseError> {
    let latency = ctx.ping().await;
    let author = ctx.author();

    let title = ":satellite: Pong!";
    let description = format!("`Gateway Latency:` **`{}ms`**", latency.as_millis());
    let embed = create_user_color_embed(author, title, &description, Colors::Orange);

    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    // For testing purposes
    let (bot_user, fish_user) = ctx
        .data()
        .bot_user_service()
        .register_or_fetch_user(author)
        .map_err(CommandError::from)?;

    Ok(())
}
