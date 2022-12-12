use twilight_model::{
    application::interaction::{Interaction, InteractionData},
    channel::message::{
        component::{ActionRow, SelectMenu, SelectMenuOption},
        Component, ReactionType,
    },
};

use super::prelude::*;
pub struct StartCommand;

#[async_trait]
impl Command for StartCommand {
    fn command_config(&self) -> CommandConfig {
        CommandConfig {
            character_required: false,
        }
    }

    fn build_command(&self, application_id: Id<ApplicationMarker>) -> CommandBuilder {
        CommandBuilder::new(application_id, "iniciar", "Inicie sua jornada em Phoenix!")
    }

    async fn run(&self, ctx: CommandContext) -> CommandResult {
        let author = ctx.author().await?;
        let user_data = ctx.db().get_user_data(author.id.to_string()).await?;
        if user_data.is_some() {
            return ctx
                .reply(Response::new_user_reply(
                    author,
                    "você já iniciou sua jornada!",
                ))
                .await;
        }

        let menu = Component::SelectMenu(SelectMenu {
            custom_id: "menu".into(),
            disabled: false,
            max_values: None,
            min_values: None,
            placeholder: None,
            options: data::classes::ALL_CLASSES
                .iter()
                .map(|c| SelectMenuOption {
                    default: false,
                    description: Some(c.description.into()),
                    emoji: Some(ReactionType::Unicode {
                        name: c.emoji.into(),
                    }),
                    label: c.name.into(),
                    value: c.name.into(),
                })
                .collect(),
        });

        let action_row = Component::ActionRow(ActionRow {
            components: vec![menu],
        });

        ctx.reply(Response {
            components: Some(vec![action_row]),
            ..Response::new_user_reply(author.clone(), "escolha sua classe:")
        })
        .await?;

        let message = ctx.fetch_reply().await?;

        let standby = ctx.standby.clone();
        let component = standby
            .wait_for_component(message.id, move |event: &Interaction| {
                event.author_id().unwrap() == author.id
            })
            .await?;

        let interaction = Box::new(component);
        let ctx = CommandContext::from_with_interaction(ctx, interaction.clone());

        let Some(InteractionData::MessageComponent(data)) = interaction.data else { return Ok(()); };

        let class =
            data::classes::get_class_by_name(data.values[0].clone()).ok_or("Invalid class")?;

        ctx.db()
            .register_user_data(author.id.to_string(), class)
            .await?;

        ctx.reply(
            Response::new_user_reply(author, f!("você iniciou sua aventura com a classe **{}**! {}\nUse **/perfil** para ver seus atributos.", class.name, class.emoji))
            .set_emoji_prefix("🗺️")
        ).await?;

        Ok(())
    }
}
