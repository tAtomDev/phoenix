use twilight_model::application::command::CommandOptionType;

use crate::{battle, discord::command::CommandOptionBuilder};

use super::prelude::*;

pub struct BattleCommand;

#[async_trait]
impl Command for BattleCommand {
    fn command_config(&self) -> CommandConfig {
        CommandConfig::default()
    }

    fn build_command(&self, application_id: Id<ApplicationMarker>) -> CommandBuilder {
        CommandBuilder::new(
            application_id,
            "batalhar",
            "Batalhe amistosamente com algum amigo!",
        )
        .add_option(
            CommandOptionBuilder::new(
                "usuário",
                "Usuário que você quer batalhar",
                CommandOptionType::User,
            )
            .set_required(true),
        )
    }

    async fn run(&self, ctx: CommandContext) -> CommandResult {
        let author = ctx.author().await?;
        let user = ctx
            .options()
            .get_user("usuário")
            .await?
            .ok_or("User expected")?;

        if user.id == author.id {
            return Ok(ctx.reply(Response::new_user_reply(author, "você nâo pode batalhar com você mesmo!").error_response()).await?);
        }

        let confirmation = ctx
            .create_confirmation(
                user.clone(),
                Response::new_user_reply(
                    user,
                    f!(
                        "você foi convidado para batalhar com **{}**! Você aceita?",
                        author.name
                    ),
                ),
            )
            .await;

        if !confirmation {
            return Ok(());
        }

        let author = ctx.author().await?;
        let user_data = ctx
            .db()
            .get_user_data(author.id.to_string())
            .await?
            .ok_or("Invalid data")?;

        let fighters = vec![
            battle::Fighter::create_from_user_data(
                "teste",
                user_data.clone(),
                author.clone(),
            )?,
            battle::Fighter::create_from_user_data(
                "teste2", user_data, author,
            )?,
        ];

        battle::util::handle_battle(&ctx, &mut battle::Battle::new(fighters))
            .await
            .unwrap();

        Ok(())
    }
}
