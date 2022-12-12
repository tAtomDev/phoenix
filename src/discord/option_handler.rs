use twilight_model::{
    application::interaction::{application_command::CommandOptionValue, InteractionData},
    user::User,
};

use crate::commands::prelude::DynamicError;

use super::command::CommandContext;

pub struct OptionHandler<'a> {
    pub ctx: &'a CommandContext,
}

impl<'a> OptionHandler<'a> {
    pub async fn get_user(
        &self,
        option_name: impl Into<String>,
    ) -> Result<Option<User>, DynamicError> {
        let option_name: String = option_name.into();
        let Some(InteractionData::ApplicationCommand(data)) = &self.ctx.interaction.data else {
            return Ok(None);
        };

        let Some(option) = data.options.iter().find(|o| o.name == option_name) else {
            return Ok(None);
        };

        Ok(match option.value.clone() {
            CommandOptionValue::User(user_id) => {
                let user = self.ctx.http.user(user_id).await?.model().await?;

                Some(user)
            }
            _ => None,
        })
    }
}
