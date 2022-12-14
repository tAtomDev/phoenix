use std::time::Duration;

use crate::battle::{self, Fighter};
use data::anomalies;

use super::prelude::*;

pub struct AdventureCommand;

#[async_trait]
impl Command for AdventureCommand {
    fn command_config(&self) -> CommandConfig {
        CommandConfig::default()
    }

    fn build_command(&self, application_id: Id<ApplicationMarker>) -> CommandBuilder {
        CommandBuilder::new(
            application_id,
            "aventura",
            "Parta em sua jornada rumo à reconstrução do mundo!",
        )
    }

    async fn run(&self, mut ctx: CommandContext) -> CommandResult {
        let author = ctx.author().await?;
        let author_id = author.id;
        let mut author_data = ctx
            .db()
            .get_user_data(&author.id.to_string())
            .await?
            .ok_or("Invalid data")?;

        if author_data.health.value < 15 {
            return ctx.reply(Response::new_user_reply(author, "você está sem vida para batalhar! Use **/descansar** antes de partir para uma nova aventura.").error_response()).await;
        }

        let author_fighter = Fighter::create_from_user_data(author.clone(), author_data.clone())?;

        let anomaly = anomalies::generate_random_anomaly(author_data.level);
        let anomaly_fighter = Fighter::create_from_anomaly(anomaly)?;

        let embed = EmbedBuilder::new()
            .set_author(EmbedAuthor {
                name: f!("{} encontrou uma anomalia!", author.name),
                icon_url: Some(author.avatar_url()),
            })
            .add_field(EmbedField {
                name: f!("{} (nível {})", anomaly.name(), anomaly.level),
                value: f!("{}", anomaly_fighter.display_full_stats()),
                inline: false,
            })
            .set_description(f!("Se vencer, você receberá:\n{}", anomaly.rewards))
            .set_image(anomaly.image())
            .set_color(Color::YELLOW)
            .set_current_timestamp();

        let confirmation = ctx
            .create_confirmation(
                author.clone(),
                Response {
                    embeds: Some(vec![embed]),
                    ..Response::new_user_reply(
                        author.clone(),
                        "você encontrou uma anomalia. Quer enfrentá-la?",
                    )
                },
            )
            .await;

        if !confirmation {
            return Ok(());
        }

        let fighters = vec![author_fighter, anomaly_fighter];

        let battle = &mut battle::Battle::new(fighters)?;

        let battle_result = battle::util::handle_battle(&ctx, battle).await?;

        let winner = battle_result.winner;
        let author_fighter = battle_result
            .all_fighters
            .iter()
            .find(|f| f.user.as_ref().map_or(false, |u| u.id == author_id))
            .ok_or("Author fighter not found")?;

        if let Some(user) = winner.user {
            if user.id == author.id {
                author_data.add_gold(anomaly.rewards.gold);
                author_data.add_xp(anomaly.rewards.xp);
                let new_level = author_data.level_up();

                ctx.send_in_channel(Response::new_user_reply(
                    author.clone(),
                    f!("você recebeu:\n{}", anomaly.rewards),
                ))
                .await?;

                if let Some(level) = new_level {
                    let ctx = ctx.clone();
                    let author = author.clone();

                    #[rustfmt::skip]
                    util::set_tokio_timeout(Duration::from_secs(3), async move {
                        ctx.send_in_channel(Response::new_user_reply(author, f!("você agora está no nível **{}**", level))).await.ok();
                    });
                }
            }
        }

        author_data.set_health(author_fighter.health.value);
        author_data.set_mana(author_fighter.mana.value);

        ctx.db().save_user_data(author_data).await?;

        Ok(())
    }
}
