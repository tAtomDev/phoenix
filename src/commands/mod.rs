use std::collections::HashMap;

use async_trait::async_trait;
use lazy_static::lazy_static;
use prelude::*;

mod battle;
mod ping;
mod profile;
mod start;

lazy_static! {
    pub static ref COMMANDS: HashMap<&'static str, Box<dyn Command + Send + Sync>> = {
        let mut map: HashMap<&'static str, Box<dyn Command + Send + Sync>> = HashMap::new();
        map.insert("ping", Box::new(ping::PingCommand));
        map.insert("perfil", Box::new(profile::ProfileCommand));
        map.insert("iniciar", Box::new(start::StartCommand));
        map.insert("batalhar", Box::new(battle::BattleCommand));

        map
    };
}

#[async_trait]
pub trait Command {
    fn command_config(&self) -> CommandConfig;
    fn build_command(&self, application_id: Id<ApplicationMarker>) -> CommandBuilder;
    async fn run(&self, mut ctx: CommandContext) -> CommandResult;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandConfig {
    pub character_required: bool,
}

impl Default for CommandConfig {
    fn default() -> Self {
        Self {
            character_required: true,
        }
    }
}

pub mod prelude {
    pub use super::Command;
    pub use super::CommandConfig;
    pub use crate::discord::command::{CommandBuilder, CommandContext};
    pub use crate::discord::Response;
    pub use crate::prelude::DynamicError;
    pub use async_trait::async_trait;
    pub(crate) use format as f;

    pub type CommandResult = Result<(), DynamicError>;

    pub use twilight_model::{
        gateway::payload::incoming::InteractionCreate,
        http::interaction::{
            InteractionResponse, InteractionResponseData, InteractionResponseType,
        },
        id::{
            marker::{ApplicationMarker, CommandVersionMarker},
            Id,
        },
    };
}
