use std::{time::Duration, pin::Pin};

use async_trait::async_trait;
use tokio_stream::{StreamExt, Timeout};
use trait_set::trait_set;
use twilight_model::{
    application::interaction::Interaction,
    id::{marker::MessageMarker, Id},
    user::User,
};
use twilight_standby::{Standby, future::WaitForComponentStream};

use crate::commands::prelude::DynamicError;

pub trait UserExtension {
    fn avatar_url(&self) -> String;
    fn mention(&self) -> String;
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

    fn mention(&self) -> String {
        format!("<@{}>", self.id)
    }
}

trait_set! {
    pub trait ComponentFilter = Fn(&Interaction) -> bool + Send + Sync + 'static;
}


#[async_trait]
pub trait StandbyExtension {
    fn create_component_stream<T: ComponentFilter>(&self, message_id: Id<MessageMarker>, duration: Duration, filter: T) -> Pin<Box<Timeout<WaitForComponentStream>>>;

    async fn wait_for_component_with_duration<T: ComponentFilter>(&self, message_id: Id<MessageMarker>, duration: Duration, filter: T) 
        -> Result<Option<Interaction>, DynamicError>;
}

#[async_trait]
impl StandbyExtension for Standby {
    fn create_component_stream<T: ComponentFilter>(&self, message_id: Id<MessageMarker>, duration: Duration, filter: T) -> Pin<Box<Timeout<WaitForComponentStream>>> {
        let stream = self
            .wait_for_component_stream(message_id, filter)
            .timeout(duration);
    
        Box::pin(stream)
    }

    async fn wait_for_component_with_duration<T: ComponentFilter>(&self, message_id: Id<MessageMarker>, duration: Duration, filter: T) 
        -> Result<Option<Interaction>, DynamicError> {
        let mut stream = self.create_component_stream(message_id, duration, filter);

        let Some(component) = stream.next().await else {
            return Ok(None);
        };

        let interaction = component?;

        Ok(Some(interaction))
    }
}