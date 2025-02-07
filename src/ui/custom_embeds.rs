use crate::enums::colors::Colors;
use crate::structures::embed_fields::EmbedFields;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedAuthor, User};

pub fn create_success_embed(author: &User, title: &str, description: &str) -> CreateEmbed {
    create_user_color_embed(author, title, description, Colors::Success)
}

pub fn create_user_color_embed(
    author: &User,
    title: &str,
    description: &str,
    color: Colors,
) -> CreateEmbed {
    create_user_embed(author, title, description, color, EmbedFields::default())
}

pub fn create_user_embed<N, V>(
    author: &User,
    title: &str,
    description: &str,
    color: Colors,
    fields: EmbedFields<N, V>,
) -> CreateEmbed
where
    N: Into<String>,
    V: Into<String>,
{
    let avatar_url = author.avatar_url().unwrap_or(author.default_avatar_url());
    let embed_author = CreateEmbedAuthor::new(author.name.clone()).icon_url(avatar_url);

    CreateEmbed::new()
        .author(embed_author)
        .title(title)
        .description(description)
        .color(color)
        .fields(fields)
}
