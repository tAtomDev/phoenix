#![allow(unused)]
use std::{sync::Arc, time::Duration};

use database::Database;
use twilight_http::{
    client::InteractionClient, response::marker::EmptyBody, Client as HttpClient,
    Response as HttpResponse,
};
use twilight_model::{
    application::{
        command::{Command, CommandOption, CommandOptionType, CommandType},
        interaction::{application_command::CommandOptionValue, Interaction, InteractionData},
    },
    channel::{
        message::{
            component::{ActionRow, Button, ButtonStyle},
            Component, ReactionType,
        },
        Message,
    },
    gateway::payload::incoming::InteractionCreate,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
    id::{
        marker::{ApplicationMarker, CommandVersionMarker, GuildMarker, MessageMarker, UserMarker},
        Id,
    },
    user::User,
};
use twilight_standby::Standby;

use crate::{config, DynamicError};

use super::{
    component::{ActionRowBuilder, ButtonBuilder},
    extensions::StandbyExtension,
    option_handler::OptionHandler,
    Response,
};

#[derive(Debug, Clone)]
pub struct CommandContext {
    pub http: Arc<HttpClient>,
    pub interaction: Box<Interaction>,
    database: Arc<Database>,
    pub standby: Arc<Standby>,
    already_replied: bool,
}

impl CommandContext {
    pub fn new(
        http: Arc<HttpClient>,
        interaction: Box<Interaction>,
        database: Arc<Database>,
        standby: Arc<Standby>,
    ) -> Self {
        Self {
            http,
            interaction,
            database,
            standby,
            already_replied: false,
        }
    }

    pub fn from_with_interaction(ctx: &CommandContext, interaction: Box<Interaction>) -> Self {
        Self {
            database: ctx.database.clone(),
            http: ctx.http.clone(),
            interaction,
            standby: ctx.standby.clone(),
            already_replied: false,
        }
    }

    pub fn client(&self) -> InteractionClient<'_> {
        self.http.interaction(self.interaction.application_id)
    }

    pub fn author_id(&self) -> Result<Id<UserMarker>, &str> {
        self.interaction
            .author_id()
            .as_ref()
            .ok_or("User ID not found")
            .cloned()
    }

    pub async fn author(&self) -> Result<User, DynamicError> {
        let id = self.author_id()?;
        let user = self.http.user(id).await?.model().await;

        match user {
            Ok(user) => Ok(user),
            _ => Err("User not found")?,
        }
    }

    pub fn db(&self) -> &database::Database {
        &self.database
    }

    pub fn options(&'_ self) -> OptionHandler<'_> {
        OptionHandler { ctx: self }
    }

    pub async fn create_confirmation(&self, user: User, response: Response) -> bool {
        let response = Response {
            components: Some(vec![ActionRowBuilder::new()
                .add_button(
                    ButtonBuilder::new()
                        .set_label("Sim")
                        .set_custom_id("yes")
                        .set_emoji(ReactionType::Unicode { name: '✅'.into() })
                )
                .add_button(
                    ButtonBuilder::new()
                        .set_label("Não")
                        .set_custom_id("no")
                        .set_emoji(ReactionType::Unicode { name: '❌'.into() })
                )
                .build()
            ]),
            ..response
        };

        let Ok(message) = self.send(response.clone()).await else {
            return false;
        };

        let standby = self.standby.clone();
        let Ok(Some(component)) = standby.wait_for_component_with_duration(message.id, Duration::from_secs(60), move |event| {
            event.author_id() == Some(user.id)
        }).await else {
            return false;
        };

        let Some(InteractionData::MessageComponent(data)) = &component.data else {
            return false;
        };

        let ctx = CommandContext::from_with_interaction(self, Box::new(component.clone()));

        ctx.update_interaction(response.remove_all_components()).await.ok();

        matches!(data.custom_id.as_str(), "yes")
    }

    pub async fn send(&self, response: Response) -> Result<Message, DynamicError> {
        if self.already_replied {
            Ok(self.send_in_channel(response).await?)
        } else {
            self.reply(response).await?;

            Ok(self.fetch_reply().await?)
        }
    }

    pub async fn update_interaction(&self, response: Response) -> Result<(), DynamicError> {
        self.client()
            .create_response(
                self.interaction.id,
                &self.interaction.token,
                &InteractionResponse {
                    kind: InteractionResponseType::UpdateMessage,
                    data: Some(response.into()),
                },
            )
            .await?;

        Ok(())
    }

    pub async fn reply(&self, response: Response) -> Result<(), DynamicError> {
        self.client()
            .create_response(
                self.interaction.id,
                &self.interaction.token,
                &InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(response.into()),
                },
            )
            .await?;

        Ok(())
    }

    pub async fn send_in_channel(&self, response: Response) -> Result<Message, DynamicError> {
        let mut message = self
            .http
            .create_message(self.interaction.channel_id.unwrap());

        let response: InteractionResponseData = response.into();

        if let Some(content) = &response.content {
            message = message.content(content)?;
        }

        if let Some(embeds) = &response.embeds {
            message = message.embeds(embeds.as_slice())?;
        }

        if let Some(components) = &response.components {
            message = message.components(components.as_slice())?;
        }

        let response = message.await?;

        Ok(response.model().await?)
    }

    pub async fn fetch_reply(&self) -> Result<Message, DynamicError> {
        let response = self
            .client()
            .response(self.interaction.token.to_string().as_str())
            .await?;

        Ok(response.model().await?)
    }

    pub async fn edit_reply(&self, response: Response) -> Result<(), DynamicError> {
        let client = self.client();
        let update = client.update_response(&self.interaction.token);

        let response: InteractionResponseData = response.into();

        update
            .content(response.content.as_deref())?
            .embeds(response.embeds.as_deref())?
            .components(response.components.as_deref())?
            .await?;

        Ok(())
    }

    pub async fn delete_message(&self, message: Message) -> Result<(), DynamicError> {
        self.http
            .delete_message(message.channel_id, message.id)
            .await?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommandBuilder {
    command: Command,
}

impl CommandBuilder {
    pub fn new<S: ToString>(
        application_id: Id<ApplicationMarker>,
        name: S,
        description: S,
    ) -> Self {
        Self {
            command: Command {
                application_id: Some(application_id),
                name: name.to_string(),
                description: description.to_string(),
                default_member_permissions: None,
                description_localizations: None,
                dm_permission: None,
                guild_id: None,
                id: None,
                kind: CommandType::ChatInput,
                name_localizations: None,
                options: vec![],
                version: Id::new(1),
            },
        }
    }

    pub fn set_guild_id(mut self, guild_id: Id<GuildMarker>) -> Self {
        self.command.guild_id = Some(guild_id);
        self
    }

    pub fn add_option(mut self, option: CommandOptionBuilder) -> Self {
        self.command.options.push(option.build());
        self
    }

    pub fn set_options(mut self, options: Vec<CommandOption>) -> Self {
        self.command.options = options;
        self
    }

    pub fn build(&self) -> Command {
        self.command.to_owned()
    }
}

pub struct CommandOptionBuilder {
    option: CommandOption,
}

impl CommandOptionBuilder {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        kind: CommandOptionType,
    ) -> Self {
        Self {
            option: CommandOption {
                name: name.into(),
                description: description.into(),
                kind,
                autocomplete: None,
                channel_types: None,
                choices: None,
                name_localizations: None,
                description_localizations: None,
                min_length: None,
                max_length: None,
                min_value: None,
                max_value: None,
                options: None,
                required: None,
            },
        }
    }

    pub fn set_required(mut self, required: bool) -> Self {
        self.option.required = Some(required);
        self
    }

    pub fn build(&self) -> CommandOption {
        self.option.to_owned()
    }
}
