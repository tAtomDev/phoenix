use super::prelude::*;
use chrono::Utc;

pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {
    fn command_config(&self) -> CommandConfig {
        CommandConfig {
            character_required: false,
        }
    }

    fn build_command(&self, application_id: Id<ApplicationMarker>) -> CommandBuilder {
        CommandBuilder::new(application_id, "ping", "Verifique se estou funcionando bem")
    }

    async fn run(&self, mut ctx: CommandContext) -> CommandResult {
        let start = Utc::now().timestamp_millis();

        ctx.reply(Response::from_string("Pong!")).await?;

        let ping = Utc::now().timestamp_millis() - start;
        ctx.edit_reply(Response::from_string(f!("Ping: {ping}ms")))
            .await?;

        Ok(())
    }
}
