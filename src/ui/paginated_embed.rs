use crate::structures::embed_fields::EmbedFields;
use crate::{PoiseContext, PoiseError};
use poise::serenity_prelude::{
    ButtonStyle, Color, ComponentInteraction, CreateActionRow, CreateButton, CreateEmbed,
    CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, ReactionType,
};
use poise::CreateReply;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct PaginatedEmbed {
    pages: Vec<EmbedPage>,
    timeout: Duration,
    current_page: usize,
}

#[derive(Debug, Clone)]
pub struct EmbedPage {
    pub title: Option<String>,
    pub description: String,
    pub fields: EmbedFields<String, String>,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonId {
    Prev,
    Next,
}

impl PaginatedEmbed {
    pub fn new(pages: Vec<EmbedPage>) -> Self {
        Self {
            pages,
            timeout: Duration::from_secs(300),
            current_page: 0,
        }
    }

    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;
        self
    }

    pub fn starting_page(mut self, page: usize) -> Self {
        self.current_page = page;
        self
    }

    fn create_navigation_buttons(&self, style: ButtonStyle, disabled: bool) -> CreateActionRow {
        let prev_button = CreateButton::new(ButtonId::Prev)
            .emoji(ReactionType::Unicode("⬅".to_string()))
            .style(style)
            .disabled(disabled);

        let next_button = CreateButton::new(ButtonId::Next)
            .emoji(ReactionType::Unicode("➡".to_string()))
            .style(style)
            .disabled(disabled);

        CreateActionRow::Buttons(vec![prev_button, next_button])
    }

    fn create_embed(&self) -> CreateEmbed {
        let page = &self.pages[self.current_page];
        let mut embed = CreateEmbed::default()
            .description(&page.description)
            .fields(page.fields.clone())
            .footer(CreateEmbedFooter::new(format!(
                "Page {}/{}",
                self.current_page + 1,
                self.pages.len()
            )));

        if let Some(title) = &page.title {
            embed = embed.title(title);
        }

        if let Some(color) = page.color {
            embed = embed.color(color);
        }

        embed
    }

    fn next_page(&mut self) {
        self.current_page = (self.current_page + 1) % self.pages.len();
    }

    fn prev_page(&mut self) {
        self.current_page = if self.current_page == 0 {
            self.pages.len() - 1
        } else {
            self.current_page - 1
        };
    }

    async fn handle_interaction(
        &mut self,
        interaction: &ComponentInteraction,
        ctx: &PoiseContext<'_>,
    ) -> Result<(), PoiseError> {
        let button_id: ButtonId = interaction.data.custom_id.as_str().into();

        match button_id {
            ButtonId::Prev => self.prev_page(),
            ButtonId::Next => self.next_page(),
        }

        let response_message = CreateInteractionResponseMessage::new()
            .embed(self.create_embed())
            .components(vec![
                self.create_navigation_buttons(ButtonStyle::Secondary, false)
            ]);

        interaction
            .create_response(
                &ctx.serenity_context(),
                CreateInteractionResponse::UpdateMessage(response_message),
            )
            .await?;

        Ok(())
    }

    fn finalize(&self) -> (CreateEmbed, CreateActionRow) {
        let mut embed = self.create_embed();
        embed = embed.color(Color::DARK_GREY);
        embed = embed.footer(CreateEmbedFooter::new("Embed timed out"));
        let buttons = self.create_navigation_buttons(ButtonStyle::Secondary, true);
        (embed, buttons)
    }

    pub async fn run(self, ctx: PoiseContext<'_>) -> Result<(), PoiseError> {
        let page_data = Arc::new(Mutex::new(self));

        let embed = page_data.lock().await.create_embed();
        let action_row = page_data
            .lock()
            .await
            .create_navigation_buttons(ButtonStyle::Secondary, false);

        let reply = CreateReply::default()
            .embed(embed)
            .components(Vec::from([action_row]));
        let reply_handle = ctx.send(reply).await?;

        let message = reply_handle.message().await?;
        let timeout_duration = page_data.lock().await.timeout;

        while let Some(interaction) = message
            .await_component_interactions(ctx)
            .timeout(timeout_duration)
            .await
        {
            let mut data = page_data.lock().await;
            data.handle_interaction(&interaction, &ctx).await?;
        }

        let data = page_data.lock().await;
        let (final_embed, final_row) = data.finalize();

        reply_handle
            .edit(
                ctx,
                CreateReply::default()
                    .embed(final_embed)
                    .components(vec![final_row]),
            )
            .await?;

        Ok(())
    }
}

impl From<&str> for ButtonId {
    fn from(s: &str) -> Self {
        match s {
            "prev" => ButtonId::Prev,
            "next" => ButtonId::Next,
            _ => panic!("Invalid button ID"),
        }
    }
}

impl Into<String> for ButtonId {
    fn into(self) -> String {
        match self {
            ButtonId::Prev => "prev".to_string(),
            ButtonId::Next => "next".to_string(),
        }
    }
}
