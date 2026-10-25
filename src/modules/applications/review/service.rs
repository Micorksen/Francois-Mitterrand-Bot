use poise::serenity_prelude as serenity;
use serenity::Mentionable;

use super::modals;
use crate::{modules::applications::config::RESPONSE_CHANNEL, Data, Error};

pub async fn handle(
    ctx: &serenity::Context,
    _data: &Data,
    interaction: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    let mut embed = serenity::builder::CreateEmbed::from(interaction.message.embeds[0].clone());

    match interaction.data.custom_id.as_str() {
        id if id.starts_with("applications:review:accept:") => {
            accept(ctx, interaction).await?;
            embed = embed.color(serenity::colours::branding::GREEN).footer(
                serenity::CreateEmbedFooter::new(format!(
                    "Acceptée par {} ({})",
                    interaction.user.name, interaction.user.id
                )),
            );
        }
        id if id.starts_with("applications:review:refuse:") => {
            refuse(ctx, interaction).await?;
            embed = embed.color(serenity::colours::branding::RED).footer(
                serenity::CreateEmbedFooter::new(format!(
                    "Refusée par {} ({})",
                    interaction.user.name, interaction.user.id
                )),
            );
        }
        _ => {}
    };

    // On change la couleur de l'embed et on retire les boutons
    // après que la candidature ait été acceptée ou refusée.
    let mut message = interaction.message.clone();
    message
        .edit(
            &ctx.http,
            serenity::EditMessage::new().embed(embed).components(&[]),
        )
        .await?;

    Ok(())
}

async fn accept(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    let Some(data) = poise::execute_modal_on_component_interaction::<modals::Accept>(
        ctx,
        interaction.clone(),
        None,
        None,
    )
    .await?
    else {
        return Ok(());
    };

    RESPONSE_CHANNEL
        .to_guild_channel(&ctx, interaction.guild_id)
        .await?
        .send_message(
            &ctx.http,
            serenity::CreateMessage::new()
                .content(format!(
                    "<@{}>",
                    interaction.data.custom_id.split(":").last().unwrap()
                ))
                .embed(
                    serenity::CreateEmbed::new()
                        .color(serenity::colours::branding::GREEN)
                        .title("Candidature acceptée !")
                        .description(
                            [
                                "Félicitations, ta candidature a été **acceptée** !",
                                format!(
                                    "{} te donne rendez-vous pour passer ton entretien oral.",
                                    interaction.user.mention()
                                )
                                .as_str(),
                            ]
                            .join("\n"),
                        )
                        .field("Date de l'entretien", data.date, true)
                        .field("Heure de l'entretien", data.time, true),
                ),
        )
        .await?;

    Ok(())
}

async fn refuse(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    let Some(data) = poise::execute_modal_on_component_interaction::<modals::Refuse>(
        ctx,
        interaction.clone(),
        None,
        None,
    )
    .await?
    else {
        return Ok(());
    };

    RESPONSE_CHANNEL
        .to_guild_channel(&ctx, interaction.guild_id)
        .await?
        .send_message(
            &ctx.http,
            serenity::CreateMessage::new()
                .content(format!(
                    "<@{}>",
                    interaction.data.custom_id.split(":").last().unwrap()
                ))
                .embed(
                    serenity::CreateEmbed::new()
                        .color(serenity::colours::branding::RED)
                        .title("Candidature refusée !")
                        .description(data.comment),
                ),
        )
        .await?;

    Ok(())
}
