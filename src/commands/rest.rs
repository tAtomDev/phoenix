use super::prelude::*;

pub struct RestCommand;

#[async_trait]
impl Command for RestCommand {
    fn command_config(&self) -> CommandConfig {
        CommandConfig::default()
    }

    fn build_command(&self, application_id: Id<ApplicationMarker>) -> CommandBuilder {
        CommandBuilder::new(
            application_id,
            "descansar",
            "Descanse após uma longa e complicada batalha",
        )
    }

    async fn run(&self, mut ctx: CommandContext) -> CommandResult {
        let author_id = ctx.author_id()?;

        let author = ctx.author().await?;
        let mut author_data = ctx
            .db()
            .get_user_data(&author.id.to_string())
            .await?
            .ok_or("Invalid data")?;

        if author_data.health.value as f32 > (author_data.health.max as f32 * 0.8) {
            return ctx
                .reply(
                    Response::new_user_reply(author, "você não precisa descansar!")
                        .error_response(),
                )
                .await;
        }

        if ctx
            .check_user_cooldown(author_id, CooldownType::Rest, Duration::minutes(20))
            .await?
            == CommandFlow::ShouldStop
        {
            return Ok(());
        }

        author_data.restore_health();
        author_data.restore_mana();

        ctx.db().save_user_data(author_data).await?;

        ctx.reply(Response::new_user_reply(author, "você descansou!").set_emoji_prefix("⚡"))
            .await?;

        Ok(())
    }
}
