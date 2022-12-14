use super::prelude::*;

pub struct RestCommand;

#[async_trait]
impl Command for RestCommand {
    fn command_config(&self) -> CommandConfig {
        CommandConfig::default()
    }

    fn build_command(&self, application_id: Id<ApplicationMarker>) -> CommandBuilder {
        CommandBuilder::new(application_id, "descansar", "Descanse após uma longa e complicada batalha")
    }

    async fn run(&self, mut ctx: CommandContext) -> CommandResult {
        let author = ctx.author().await?;
        let mut author_data = ctx
            .db()
            .get_user_data(author.id.to_string())
            .await?
            .ok_or("Invalid data")?;

        author_data.restore_health();
        author_data.restore_mana();

        ctx.db().save_user_data(author_data).await?;

        ctx.reply(Response::new_user_reply(author, "você descansou!").set_emoji_prefix("⚡")).await?;

        Ok(())
    }
}