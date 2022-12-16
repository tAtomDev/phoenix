use std::time::Duration;

use crate::battle::{self, Fighter};
use data::{anomalies, regions::RegionType};
use database::user_model::Region;
use rand::{thread_rng, Rng};

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

        if author_data.journey.current_region.region_type == RegionType::City {
            let confirmation = ctx.create_confirmation(
                author.clone(),
                Response::new_user_reply(
                    author.clone(),
                    f!(
                        "se você sair em uma aventura, você não poderá voltar de novo para **{}**!\nVocê quer mesmo ir para uma aventura agora?", 
                        author_data.journey.current_region.name
                    )
                ).set_emoji_prefix("🗺️")
            ).await;

            if !confirmation {
                return Ok(());
            }

            let new_region = Region::generate_random_from_journey(author_data.journey.clone());

            author_data.travel_distance(thread_rng().gen_range(0.65..0.7));
            author_data.travel_to_region(new_region.clone());

            ctx.db().save_user_data(author_data).await?;

            ctx.send_in_channel(
                Response::new_user_reply(author, f!("você saiu da sua cidade e caminhou até chegar em **{}**!", new_region.name))
                .set_emoji_prefix(new_region.emoji())
            ).await?;

            return Ok(());
        }

        let author_fighter = Fighter::create_from_user_data(author.clone(), author_data.clone())?;

        let anomaly = anomalies::generate_random_anomaly(author_data.level, author_data.journey.current_region.region_type);
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
                let distance = thread_rng().gen_range(0.2..0.4) as f32;
                author_data.travel_distance(distance);

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
                    util::set_tokio_timeout(Duration::from_secs(1), async move {
                        ctx.send_in_channel(
                            Response::new_user_reply(author, f!("você agora está no nível **{}**", level))
                        ).await.ok();
                    });
                }

                if author_data.journey.region_history.len() == 0 || author_data.journey.total_traveled > (author_data.journey.current_region.distance + thread_rng().gen_range(0.8..1.2)) {
                    let ctx = ctx.clone();
                    let author = author.clone();
                    let new_region = Region::generate_random_from_journey(author_data.journey.clone());

                    author_data.travel_to_region(new_region.clone());

                    let clone_region = new_region.clone();
                    #[rustfmt::skip]
                    util::set_tokio_timeout(Duration::from_secs(2), async move {
                        ctx.send_in_channel(
                            Response::new_user_reply(author, f!("você vagou e chegou em **{}**!", clone_region.name))
                            .set_emoji_prefix(clone_region.emoji())
                        ).await.ok();
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
