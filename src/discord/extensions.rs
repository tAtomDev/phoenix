use std::time::Duration;

use async_trait::async_trait;
use tokio_stream::StreamExt;
use twilight_model::{
    application::interaction::Interaction,
    id::{marker::MessageMarker, Id},
    user::User,
};
use twilight_standby::Standby;

use crate::commands::prelude::DynamicError;

pub trait UserExtension {
    fn avatar_url(&self) -> String;
}

impl UserExtension for User {
    fn avatar_url(&self) -> String {
        let Some(avatar) = self.avatar else {
            return "https://external-preview.redd.it/fauTrGFvbnTjWM6A6AC8sGqohLQxKHQTfZjhtPbWY7g.jpg?auto=webp&s=5d8e36356dead73ec2e624e41659d411b5fbca53".into();
        };

        format!(
            "https://cdn.discordapp.com/avatars/{}/{}.png",
            self.id, avatar
        )
    }
}

#[async_trait]
pub trait StandbyExtension {
    async fn wait_for_component_with_duration<T: Fn(&Interaction) -> bool + Send + Sync + 'static>(
        &self,
        message_id: Id<MessageMarker>,
        duration: Duration,
        filter: T,
    ) -> Result<Option<Interaction>, DynamicError>;
}

#[async_trait]
impl StandbyExtension for Standby {
    async fn wait_for_component_with_duration<
        T: Fn(&Interaction) -> bool + Send + Sync + 'static,
    >(
        &self,
        message_id: Id<MessageMarker>,
        duration: Duration,
        filter: T,
    ) -> Result<Option<Interaction>, DynamicError> {
        let stream = self
            .wait_for_component_stream(message_id, move |event: &Interaction| filter(event))
            .timeout(duration);

        tokio::pin!(stream);

        let Some(component) = stream.next().await else {
            return Ok(None);
        };

        let interaction = component?;

        Ok(Some(interaction))
    }
}
