use data::anomalies::get_anomaly_from_type;

use crate::{battle::Fighter, discord::pagination::EmbedPagination};

use super::prelude::*;

pub struct BestiaryCommand;

#[async_trait]
impl Command for BestiaryCommand {
    fn command_config(&self) -> CommandConfig {
        CommandConfig::default()
    }

    fn build_command(&self, application_id: Id<ApplicationMarker>) -> CommandBuilder {
        CommandBuilder::new(
            application_id,
            "bestiário",
            "Veja informações sobre as anomalias que você já enfrentou",
        )
    }

    async fn run(&self, mut ctx: CommandContext) -> CommandResult {
        let author = ctx.author().await?;
        let author_data = ctx
            .db()
            .get_user_data(&author.id.to_string())
            .await?
            .ok_or("Invalid data")?;

        if author_data.bestiary.len() == 0 {
            ctx.send(
                Response::new_user_reply(
                    author,
                    "você ainda não venceu nenhuma anomalia para colocar no bestiário!",
                )
                .error_response(),
            )
            .await?;

            return Ok(());
        }

        let mut pages: Vec<EmbedBuilder> = Vec::new();
        for entry in author_data.bestiary {
            let anomaly = get_anomaly_from_type(entry.anomaly).ok_or("Invalid anomaly")?;
            let anomaly_fighter = Fighter::create_from_anomaly(anomaly)?;

            let embed = EmbedBuilder::new()
                .set_author(EmbedAuthor {
                    name: f!("Bestiário de {}", author.name),
                    icon_url: Some(author.avatar_url()),
                })
                .set_color(Color::ORANGE)
                .set_thumbnail(anomaly.image())
                .set_title(f!("{}", anomaly.name()))
                .set_description(f!(
                    "Você derrotou essa anomalia **{}** vezes e foi derrotado **{}** vezes.",
                    entry.wins,
                    entry.loses
                ))
                .add_field(EmbedField {
                    name: f!("🟢 Atributos Base"),
                    value: f!("{}", anomaly_fighter.display_full_stats()),
                    inline: false,
                })
                .set_current_timestamp();

            pages.push(embed);
        }

        EmbedPagination::new(ctx, pages).send().await?;

        Ok(())
    }
}
