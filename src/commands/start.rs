use std::time::Duration;

use twilight_model::{application::interaction::InteractionData, channel::message::ReactionType};

use crate::discord::extensions::StandbyExtension;

use super::prelude::*;
pub struct StartCommand;

#[async_trait]
impl Command for StartCommand {
    fn command_config(&self) -> CommandConfig {
        CommandConfig {
            character_required: false,
        }
    }

    fn build_command(&self, application_id: Id<ApplicationMarker>) -> CommandBuilder {
        CommandBuilder::new(application_id, "iniciar", "Inicie sua jornada em Phoenix!")
    }

    async fn run(&self, mut ctx: CommandContext) -> CommandResult {
        let author = ctx.author().await?;
        let user_data = ctx.db().get_user_data(&author.id.to_string()).await?;
        if user_data.is_some() {
            return ctx
                .reply(Response::new_user_reply(
                    author,
                    "você já iniciou sua jornada!",
                ))
                .await;
        }

        let embed = EmbedBuilder::new()
            .set_color(Color::BLURPLE)
            .set_author(EmbedAuthor {
                name: f!("{}, escolha sua classe!", author.name),
                icon_url: Some(author.avatar_url()),
            })
            .add_fields(
                &mut data::classes::ALL_CLASSES
                    .iter()
                    .map(|c| EmbedField {
                        name: f!("{}, {}", c.emoji, c.name),
                        value: c.description.into(),
                        inline: false,
                    })
                    .collect(),
            )
            .set_current_timestamp();

        let action_row = ActionRowBuilder::new()
            .add_buttons(
                data::classes::ALL_CLASSES
                    .iter()
                    .map(|c| {
                        ButtonBuilder::new()
                            .set_custom_id(c.name)
                            .set_label(c.name)
                            .set_emoji(ReactionType::Unicode {
                                name: c.emoji.into(),
                            })
                    })
                    .collect(),
            )
            .build();

        ctx.reply(Response {
            components: Some(vec![action_row]),
            embeds: Some(vec![embed]),
            ..Response::new_user_reply(author.clone(), "escolha sua classe:")
        })
        .await?;

        let message = ctx.fetch_reply().await?;

        let standby = ctx.standby.clone();
        let Ok(Some(component)) = standby.wait_for_component_with_duration(message.id, Duration::from_secs(120), move |event| {
            event.author_id() == Some(author.id)
        }).await else {
            return Ok(());
        };

        let interaction = Box::new(component);
        let mut ctx = CommandContext::from_with_interaction(&ctx, interaction.clone());

        let Some(InteractionData::MessageComponent(data)) = interaction.data else {
            Err("Data not found")? 
        };

        let class = data::classes::get_class_by_name(data.custom_id).ok_or("Invalid class")?;

        ctx.db()
            .register_user_data(&author.id.to_string(), class)
            .await?;

        ctx.reply(
            Response::new_user_reply(author, f!("você iniciou sua aventura com a classe **{}**! {}\nUse **/perfil** para ver seus atributos.", class.name, class.emoji))
            .set_emoji_prefix("🗺️")
        ).await?;

        Ok(())
    }
}
