use super::prelude::*;

pub struct OwnerCommand;

#[async_trait]
impl Command for OwnerCommand {
    fn command_config(&self) -> CommandConfig {
        CommandConfig {
            character_required: false,
        }
    }

    fn build_command(&self, application_id: Id<ApplicationMarker>) -> CommandBuilder {
        CommandBuilder::new(application_id, "owner", "OWNER COMMANDS")
    }

    async fn run(&self, mut ctx: CommandContext) -> CommandResult {
        ctx.db().delete_all_cooldowns().await?;

        ctx.reply(Response::from_string(
            "todos os cooldowns foram reiniciados!",
        ))
        .await?;
        Ok(())
    }
}
