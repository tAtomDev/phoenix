use std::time::Duration;

use tokio_stream::StreamExt;
use twilight_model::{
    application::interaction::{Interaction, InteractionData},
    channel::message::{Component, ReactionType},
};
use util::Pagination;

use crate::commands::prelude::DynamicError;

use super::{
    command::CommandContext,
    component::{ActionRowBuilder, ButtonBuilder},
    embed::*,
    extensions::StandbyExtension,
    Response,
};

pub struct EmbedPagination {
    ctx: CommandContext,
    pagination: Pagination<EmbedBuilder>,
}

impl EmbedPagination {
    pub fn new(ctx: CommandContext, pages: Vec<EmbedBuilder>) -> Self {
        Self {
            ctx,
            pagination: Pagination::new(pages),
        }
    }

    pub fn _stop(&mut self) {
        self.pagination.active = false;
    }

    fn generate_embed(&self) -> EmbedBuilder {
        let embed = &mut self.pagination.get_current_page().clone();

        embed.add_footer_text(format!(
            "Página {} de {}",
            self.pagination.page + 1,
            self.pagination.pages.len()
        ))
    }

    fn generate_components(&self) -> Vec<Component> {
        if self.pagination.pages.len() < 2 {
            return vec![];
        }

        vec![ActionRowBuilder::new()
            .add_button(ButtonBuilder::new().set_custom_id("previous").set_emoji(
                ReactionType::Unicode {
                    name: "◀️".into()
                },
            ))
            .add_button(ButtonBuilder::new().set_custom_id("next").set_emoji(
                ReactionType::Unicode {
                    name: "▶️".into()
                },
            ))
            .build()]
    }

    fn generate_response(&self) -> Response {
        Response {
            embeds: Some(vec![self.generate_embed()]),
            components: Some(self.generate_components()),
            ..Default::default()
        }
    }

    pub async fn send(&mut self) -> Result<(), DynamicError> {
        let author_id = self.ctx.author_id()?;
        let message = self.ctx.send(self.generate_response()).await?;

        let standby = self.ctx.standby.clone();
        let mut stream = standby.create_component_stream(
            message.id,
            Duration::from_secs(300),
            move |event: &Interaction| event.author_id() == Some(author_id),
        );

        while let Some(Ok(collected)) = stream.next().await {
            let Some(InteractionData::MessageComponent(data)) = &collected.data else {
                break;
            };

            if data.custom_id == "next" {
                self.pagination.goto_next_page();
            } else if data.custom_id == "previous" {
                self.pagination.goto_previous_page();
            }

            if !self.pagination.active {
                break;
            }

            let ctx = CommandContext::from_with_interaction(&self.ctx, Box::new(collected));
            ctx.update_interaction(self.generate_response()).await?;
        }

        Ok(())
    }
}
