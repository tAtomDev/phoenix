use twilight_model::{
    channel::message::{Component, MessageFlags},
    http::interaction::InteractionResponseData,
    user::User,
};

use self::embed::EmbedBuilder;

pub mod command;
pub mod embed;
pub mod extensions;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Response {
    pub content: Option<String>,
    pub embeds: Option<Vec<EmbedBuilder>>,
    pub flags: Option<MessageFlags>,
    pub components: Option<Vec<Component>>,
}

impl From<Response> for InteractionResponseData {
    fn from(response: Response) -> Self {
        Self {
            content: response.content,
            embeds: response
                .embeds
                .map(|vec| vec.iter().cloned().map(|e| e.build()).collect()),
            flags: response.flags,
            components: response.components,
            ..Default::default()
        }
    }
}

impl Response {
    pub fn new_user_reply(user: User, string: impl Into<String>) -> Response {
        Response::from_string(format!("**{}**, {}", user.name, string.into()))
    }

    pub fn from_string(string: impl Into<String>) -> Response {
        Response {
            content: Some(string.into()),
            ..Default::default()
        }
    }

    pub fn from_embeds(embeds: Vec<EmbedBuilder>) -> Response {
        Response {
            embeds: Some(embeds),
            ..Default::default()
        }
    }

    pub fn set_emoji_prefix(self, emoji: impl Into<String>) -> Response {
        Response {
            content: self
                .content
                .map(|c| format!("{} **|** {}", emoji.into(), c)),
            ..self
        }
    }

    pub fn error_response(self) -> Response {
        self.set_emoji_prefix(":x:")
    }

    pub fn success_response(self) -> Response {
        self.set_emoji_prefix("✅")
    }
}
