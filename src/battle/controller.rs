use std::time::Duration;

use async_recursion::async_recursion;
use twilight_model::{
    application::interaction::{Interaction, InteractionData},
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        Component, ReactionType,
    },
};

use crate::{
    commands::prelude::{CommandContext, DynamicError, Response},
    discord::{
        component::{ActionRowBuilder, ButtonBuilder},
        embed::{EmbedAuthor, EmbedBuilder, EmbedField},
        extensions::{StandbyExtension, UserExtension},
        pagination::EmbedPagination,
    },
};
use util::Color;

use super::{ActionType, Battle, BattleResult, ALL_ACTION_TYPES};

fn get_battle_embed(battle: &Battle) -> EmbedBuilder {
    let current_fighter = battle.current_fighter();

    EmbedBuilder::new()
        .set_color(Color::BLURPLE)
        .set_author(EmbedAuthor {
            name: format!("Rodada de {}", current_fighter.name),
            icon_url: current_fighter.user.as_ref().map(|u| u.avatar_url()),
        })
        .add_fields(
            &mut battle
                .fighters
                .iter()
                .cloned()
                .map(|f| {
                    let target = battle.fighters.get(f.target_index.unwrap()).unwrap();

                    EmbedField {
                        name: f.name.to_string(),
                        value: f.display_full_stats_with_target(target),
                        inline: true,
                    }
                })
                .collect(),
        )
        .set_current_timestamp()
}

fn get_battle_action_components(_battle: &Battle) -> Component {
    ActionRowBuilder::new()
        .add_buttons(
            ALL_ACTION_TYPES
                .iter()
                .copied()
                .map(|a| {
                    ButtonBuilder::new()
                        .set_custom_id(a.name())
                        .set_emoji(ReactionType::Unicode {
                            name: a.emoji().into(),
                        })
                        .set_label(a.name())
                })
                .collect(),
        )
        .build()
}

async fn wait_for_battle_action(
    ctx: &CommandContext,
    battle: Battle,
) -> Result<Option<ActionType>, DynamicError> {
    let author_id = ctx.author_id()?;

    if let Some(anomaly) = battle.current_fighter().anomaly {
        return Ok(Some(battle.current_fighter().choose_action_type(&battle)));
    }

    let message = ctx
        .send_in_channel(Response {
            embeds: Some(vec![get_battle_embed(&battle)]),
            components: Some(vec![get_battle_action_components(&battle)]),
            ..Default::default()
        })
        .await?;

    let user = battle.current_fighter().user.clone().unwrap();

    let standby = ctx.standby.clone();
    let Ok(Some(component)) = standby.wait_for_component_with_duration(message.id, Duration::from_secs(500), move |event: &Interaction| {
        event.author_id() == Some(user.id)
    }).await else {
        return Ok(None);
    };

    let Some(InteractionData::MessageComponent(data)) = &component.data else {
        return Ok(None);
    };

    let action = ActionType::from_name(&data.custom_id).ok_or("Invalid action")?;

    let interaction = Box::new(component);
    let ctx = CommandContext::from_with_interaction(ctx, interaction.clone());

    ctx.update_interaction(Response::from_embeds(vec![get_battle_embed(&battle)]))
        .await
        .ok();
    ctx.delete_message(message).await.ok();

    Ok(Some(action))
}

#[async_recursion]
pub async fn handle_battle(
    ctx: &CommandContext,
    battle: &mut Battle,
) -> Result<BattleResult, DynamicError> {
    let action = wait_for_battle_action(ctx, battle.clone())
        .await?
        .ok_or("Invalid action")?;

    let round = battle.run_action(action);

    let message = ctx
        .send_in_channel(Response::from_embeds(vec![round.into()]))
        .await
        .ok();

    if let Some(message) = message {
        let ctx_clone = ctx.clone();
        util::set_tokio_timeout(Duration::from_secs(10), async move {
            ctx_clone.delete_message(message).await.ok();
        });
    }

    tokio::time::sleep(Duration::from_secs(1)).await;

    check_or_handle_battle(ctx, battle).await
}

async fn check_or_handle_battle(
    ctx: &CommandContext,
    battle: &mut Battle,
) -> Result<BattleResult, DynamicError> {
    let Some(winner) = &battle.winner else {
        return handle_battle(ctx, battle).await;
    };

    let embed = EmbedBuilder::new()
        .set_color(Color::GREEN)
        .set_author(EmbedAuthor {
            name: format!("{} venceu!", winner.name),
            icon_url: winner.user.clone().map(|u| u.avatar_url()),
        })
        .set_thumbnail(winner.image())
        .set_description(format!(
            "{} venceu com {} vida restando!",
            winner.name, winner.health
        ))
        .add_field(EmbedField {
            name: "📜 Última ação:".into(),
            value: battle
                .rounds
                .last()
                .ok_or("Last round not found")?
                .messages
                .last()
                .unwrap_or(&"?".to_string())
                .to_string(),
            inline: true,
        })
        .set_current_timestamp();

    let mut embeds: Vec<EmbedBuilder> = vec![embed];
    let rounds = battle.rounds.clone();
    for i in (0..rounds.len()).step_by(3) {
        let mut embed = EmbedBuilder::new()
            .set_author(EmbedAuthor {
                name: "Histórico da batalha".to_string(),
                icon_url: Some(winner.image()),
            })
            .set_thumbnail(winner.image())
            .set_color(Color::LIGHT_ORANGE)
            .set_current_timestamp();

        for j in 0..3 {
            let Some(round) = rounds.get(i + j) else {
                break;
            };

            embed.add_field(EmbedField {
                name: format!(
                    "- **`#{}`**: {} usou {}",
                    (i + j),
                    round.fighter.name,
                    round.action.action_type.name()
                ),
                value: round.messages.join("\n") + "\n",
                inline: false,
            });
        }

        embeds.push(embed);
    }

    let ctx_clone = ctx.clone();
    tokio::spawn(async move {
        EmbedPagination::new(ctx_clone, embeds).send().await.ok();
    });

    //ctx.send_in_channel(Response::from_embeds(vec![embed]))
    //    .await?;

    Ok(BattleResult {
        battle: battle.to_owned(),
        all_fighters: battle.fighters.clone(),
        defeated_fighters: battle
            .fighters
            .iter()
            .cloned()
            .filter(|f| f.health.value == 0)
            .collect(),
        winner: winner.to_owned(),
    })
}
