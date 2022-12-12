use crate::{
    discord::{
        embed::{EmbedAuthor, EmbedBuilder, EmbedField},
        extensions::UserExtension,
    },
    util::Color,
};

use super::prelude::*;

pub struct ProfileCommand;

#[async_trait]
impl Command for ProfileCommand {
    fn command_config(&self) -> CommandConfig {
        CommandConfig::default()
    }

    fn build_command(&self, application_id: Id<ApplicationMarker>) -> CommandBuilder {
        CommandBuilder::new(application_id, "perfil", "Veja seu perfil de aventureiro")
    }

    async fn run(&self, ctx: CommandContext) -> CommandResult {
        let author = ctx.author().await?;
        let user_data = ctx
            .db()
            .get_user_data(author.id.to_string())
            .await?
            .ok_or("User data not found")?;

        let class = data::classes::get_class_by_type(user_data.class).ok_or("Invalid class")?;

        let embed = EmbedBuilder::new()
            .set_color(Color::BLUE)
            .set_author(EmbedAuthor {
                name: author.name.clone(),
                icon_url: Some(author.avatar_url()),
            })
            .set_thumbnail(author.avatar_url())
            .add_field(EmbedField {
                name: f!("{} Classe", class.emoji),
                value: f!("**{}**", class.name),
                inline: true,
            })
            .add_field(EmbedField {
                name: "🪙 Ouro".into(),
                value: f!("{}", user_data.gold),
                inline: true,
            })
            .add_field(EmbedField {
                name: "🔹 Experiência".into(),
                value: f!("XP: **{}**/{}\nNível: **{}**", user_data.xp, user_data.xp_required_for_level_up(), user_data.level),
                inline: true,
            })
            .add_field(EmbedField {
                name: "❤️ Vida".into(),
                value: f!("**{}**/{}", user_data.health.value, user_data.health.max),
                inline: true,
            })
            .add_field(EmbedField {
                name: "🌀 Mana".into(),
                value: f!("**{}**/{}", user_data.mana.value, user_data.mana.max),
                inline: true,
            })
            .add_field(EmbedField {
                name: "💪 Força".into(),
                value: f!("{}", user_data.strength),
                inline: true,
            })
            .set_current_timestamp();

        ctx.reply(Response::from_embeds(vec![embed])).await?;

        Ok(())
    }
}
