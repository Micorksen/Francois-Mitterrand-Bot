use poise::serenity_prelude as serenity;

use crate::{Data, Error};

pub async fn open(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    let Some(guild_id) = interaction.guild_id else {
        return Ok(());
    };

    // Vérifie si l'utilisateur a déjà un ticket ouvert.
    let channels = guild_id.channels(&ctx.http).await?;
    let ticket = channels.iter().find(|c| {
        c.parent_id == Some(data.config.ticketing.category)
            && c.base.kind == serenity::ChannelType::Text
            && c.base.name == interaction.user.id.to_string()
    });

    if let Some(ticket) = ticket {
        interaction
            .create_response(
                &ctx.http,
                serenity::CreateInteractionResponse::Message(
                    serenity::CreateInteractionResponseMessage::new()
                        .ephemeral(true)
                        .embed(
                            serenity::CreateEmbed::new()
                                .color(data.config.common.primary_color)
                                .title("Tickets")
                                .description(
                                    [
                                        "Il semblerait que tu aies déjà un ticket d'ouvert.",
                                        format!("<#{}>", ticket.id).as_str(),
                                    ]
                                    .join("\n"),
                                ),
                        ),
                ),
            )
            .await?;

        return Ok(());
    }

    interaction
        .create_response(&ctx.http, serenity::CreateInteractionResponse::Acknowledge)
        .await?;

    // On créé le ticket et on envoie le message de bienvenue.
    let mut permissions = vec![
        serenity::PermissionOverwrite {
            allow: serenity::Permissions::VIEW_CHANNEL,
            deny: serenity::Permissions::empty(),
            kind: serenity::PermissionOverwriteType::Member(interaction.user.id),
        },
        serenity::PermissionOverwrite {
            allow: serenity::Permissions::VIEW_CHANNEL,
            deny: serenity::Permissions::empty(),
            kind: serenity::PermissionOverwriteType::Member(ctx.cache.current_user().id),
        },
        serenity::PermissionOverwrite {
            allow: serenity::Permissions::empty(),
            deny: serenity::Permissions::VIEW_CHANNEL,
            kind: serenity::PermissionOverwriteType::Role(guild_id.everyone_role()),
        },
    ];

    for role in &data.config.ticketing.support_roles {
        permissions.push(serenity::PermissionOverwrite {
            allow: serenity::Permissions::VIEW_CHANNEL,
            deny: serenity::Permissions::empty(),
            kind: serenity::PermissionOverwriteType::Role(*role),
        });
    }

    let ticket = guild_id
        .create_channel(
            &ctx.http,
            serenity::CreateChannel::new(interaction.user.id.to_string())
                .kind(serenity::ChannelType::Text)
                .category(data.config.ticketing.category)
                .permissions(permissions),
        )
        .await?;

    let message = ticket
        .send_message(
            &ctx.http,
            serenity::CreateMessage::new()
                .content("@everyone")
                .embed(
                    serenity::CreateEmbed::new()
                        .color(data.config.common.primary_color)
                        .title("Tickets")
                        .description(
                            [
                                "Merci d'avoir ouvert un ticket.",
                                "Nous allons te répondre dans les plus brefs délais.",
                            ]
                            .join("\n"),
                        ),
                )
                .button(
                    serenity::CreateButton::new("ticketing:delete")
                        .emoji(serenity::ReactionType::Unicode("🗑️".parse()?))
                        .label("Supprimer")
                        .style(serenity::ButtonStyle::Danger),
                ),
        )
        .await?;

    message.pin(&ctx.http, None).await?;

    Ok(())
}

pub async fn delete(
    ctx: &serenity::Context,
    _data: &Data,
    interaction: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    interaction.channel_id.delete(&ctx.http, None).await?;
    Ok(())
}
