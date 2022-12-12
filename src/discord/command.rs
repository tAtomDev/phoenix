#![allow(unused)]
use std::sync::Arc;

use database::Database;
use twilight_http::{
    client::InteractionClient, response::marker::EmptyBody, Client as HttpClient,
    Response as HttpResponse,
};
use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::Interaction,
    },
    channel::Message,
    gateway::payload::incoming::InteractionCreate,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
    id::{
        marker::{ApplicationMarker, CommandVersionMarker, GuildMarker, UserMarker},
        Id,
    },
    user::User,
};
use twilight_standby::Standby;

use crate::{config, DynamicError};

use super::Response;

#[derive(Debug, Clone)]
pub struct CommandContext {
    http: Arc<HttpClient>,
    interaction: Box<Interaction>,
    database: Arc<Database>,
    pub standby: Arc<Standby>,
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
        }
    }

    pub fn from_with_interaction(ctx: CommandContext, interaction: Box<Interaction>) -> Self {
        Self {
            database: ctx.database.clone(),
            http: ctx.http.clone(),
            interaction,
            standby: ctx.standby.clone(),
        }
    }

    pub fn client(&self) -> InteractionClient<'_> {
        self.http
            .interaction(self.interaction.application_id.clone())
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

    pub async fn reply_in_channel(&self, response: Response) -> Result<(), DynamicError> {
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

        message.await?;

        Ok(())
    }

    pub async fn fetch_reply(&self) -> Result<Message, DynamicError> {
        let response = self
            .client()
            .response(&self.interaction.token.to_string().as_str())
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

    pub fn set_guild_id(&mut self, guild_id: Id<GuildMarker>) -> &mut Self {
        self.command.guild_id = Some(guild_id);
        self
    }

    pub fn build(&self) -> Command {
        self.command.to_owned()
    }
}
