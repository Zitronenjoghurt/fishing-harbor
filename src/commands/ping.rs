use crate::enums::colors::Colors;
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

    Ok(())
}
