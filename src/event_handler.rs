use std::sync::Arc;

use twilight_http::Client as HttpClient;
use twilight_model::{
    application::interaction::InteractionType,
    gateway::{
        event::shard::Connected,
        payload::incoming::{InteractionCreate, Ready},
    },
};

use crate::{command_handler::CommandHandler, prelude::DynamicError};

pub struct EventHandler {
    http: Arc<HttpClient>,
    command_handler: Arc<CommandHandler>,
}

impl EventHandler {
    pub fn new(http: Arc<HttpClient>, command_handler: Arc<CommandHandler>) -> Self {
        Self {
            http,
            command_handler,
        }
    }

    pub async fn ready(&self, ready: Box<Ready>) -> Result<(), DynamicError> {
        self.command_handler
            .register_commands(ready.application.id, self.http.clone())
            .await;
        tracing::info!("{} is ready!", ready.user.name);
        Ok(())
    }

    pub async fn shard_connected(&self, connected: Connected) -> Result<(), DynamicError> {
        tracing::info!("Shard {} was connected.", connected.shard_id);
        Ok(())
    }

    pub async fn interaction_create(
        &self,
        interaction: Box<InteractionCreate>,
    ) -> Result<(), DynamicError> {
        if interaction.kind != InteractionType::ApplicationCommand {
            return Ok(());
        }

        if let Err(err) = self
            .command_handler
            .execute_command(interaction, self.http.clone())
            .await
        {
            tracing::error!("{}", err);
            return Err(err);
        }

        Ok(())
    }
}
