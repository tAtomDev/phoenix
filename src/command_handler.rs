use std::sync::Arc;

use crate::{
    commands::{
        prelude::{CommandBuilder, CommandContext, EmbedField, EmbedFooter, Response},
        COMMANDS,
    },
    config,
    discord::{
        embed::{EmbedAuthor, EmbedBuilder},
        extensions::UserExtension,
    },
    prelude::DynamicError,
};
use database::Database;
use twilight_http::{client::InteractionClient, Client as HttpClient};
use twilight_model::{
    application::{command::Command as APICommand, interaction::InteractionData},
    gateway::payload::incoming::InteractionCreate,
    id::{
        marker::{ApplicationMarker, GuildMarker},
        Id,
    },
};
use twilight_standby::Standby;
use util::*;

pub struct CommandHandler {
    pub database: Arc<Database>,
    pub standby: Arc<Standby>,
}

impl CommandHandler {
    pub async fn execute_command(
        &self,
        interaction: Box<InteractionCreate>,
        http: Arc<HttpClient>,
    ) -> Result<(), DynamicError> {
        let data = interaction
            .data
            .clone()
            .and_then(|d| match d {
                InteractionData::ApplicationCommand(data) => Some(data),
                _ => None,
            })
            .ok_or("Data not found")?;

        let mut ctx = CommandContext::new(
            http.clone(),
            Box::new(interaction.0),
            self.database.clone(),
            self.standby.clone(),
        );

        let author = ctx.author().await?;

        let command = COMMANDS
            .get(data.name.as_str())
            .ok_or("Command not found")?;

        let config = command.command_config();

        if config.character_required
            && ctx
                .db()
                .get_user_data(&author.id.to_string())
                .await?
                .is_none()
        {
            return ctx.reply(
                Response::new_user_reply(ctx.author().await?, "você precisa usar **/iniciar** para começar sua aventura antes de usar esse comando!")
                .error_response()
            ).await;
        }

        // Error handling & run command
        if let Err(err) = command.run(ctx.clone()).await {
            let err = format!("```rs\n{:?}\n```", err);
            if err
                .to_lowercase()
                .contains("interaction has already been acknowledged")
                || err.to_lowercase().contains("unknown interaction")
            {
                return Ok(());
            }

            let mut embed = EmbedBuilder::new()
                .set_author(EmbedAuthor {
                    name: "Algo deu errado!".into(),
                    icon_url: Some(author.avatar_url()),
                })
                .set_color(Color::RED)
                .set_description(err)
                .add_field(EmbedField {
                    name: "Informações".into(),
                    value: format!(
                        "Comando: `{}`\nOpções: `{:?}`\nID do usuário: `{}`",
                        data.name,
                        data.options,
                        ctx.author_id()?
                    ),
                    inline: true,
                })
                .set_current_timestamp();

            let error_code = format!("{}#{}", author.id, chrono::Utc::now().timestamp_micros());

            let embed = embed.set_footer(EmbedFooter {
                text: format!("{}", error_code),
                icon_url: None,
            });

            ctx.http
                .execute_webhook(
                    Id::new(1052621285747859587),
                    "pFpSG-HdqhOCzJg8bX9LMR-1pM2Fn21HnjiVVT6uTkQlVCDWR9zFkP_X8aIHtOoeldVI",
                )
                .embeds(&[embed.build()])?
                .await?;

            ctx.send_in_channel(Response::new_user_reply(
                author,
                format!(
                    "algo deu errado!\nCódigo do erro:\n```cs\n{}\n```",
                    error_code
                ),
            ))
            .await?;
        };

        Ok(())
    }

    pub async fn register_commands(
        &self,
        application_id: Id<ApplicationMarker>,
        http: Arc<HttpClient>,
    ) {
        let mut commands: Vec<CommandBuilder> = {
            let mut commands = Vec::new();
            for (_, command) in COMMANDS.iter() {
                commands.push(command.build_command(application_id));
            }

            commands
        };
        let guild_id = match config::CANARY {
            true => Some(Id::new(config::CANARY_GUILD_ID)),
            false => None,
        };

        self.register_http_commands(
            guild_id,
            commands
                .iter_mut()
                .map(|c| {
                    if let Some(guild_id) = guild_id {
                        *c = c.clone().set_guild_id(guild_id);
                    }

                    let build = c.build();
                    tracing::info!(
                        "Registering command {}{}",
                        build.name,
                        if config::CANARY { " (CANARY)" } else { "" }
                    );

                    build
                })
                .collect::<Vec<APICommand>>()
                .as_slice(),
            http.interaction(application_id),
        )
        .await;
    }

    async fn register_http_commands<'a>(
        &self,
        guild_id: Option<Id<GuildMarker>>,
        commands: &[APICommand],
        interaction: InteractionClient<'a>,
    ) {
        match guild_id {
            Some(guild_id) => {
                interaction
                    .set_guild_commands(guild_id, commands)
                    .await
                    .expect("Failed to register guild commands");
            }
            _ => {
                interaction
                    .set_global_commands(commands)
                    .await
                    .expect("Failed to register global commands");
            }
        };
    }
}
