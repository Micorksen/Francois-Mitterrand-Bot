use poise::serenity_prelude as serenity;

use crate::{Context, Error};

/// Signaler un message.
#[poise::command(
    context_menu_command = "Signaler un message",
    ephemeral,
    interaction_context = "Guild"
)]
pub async fn signaler_un_message(
    ctx: Context<'_>,
    message: serenity::Message,
) -> Result<(), Error> {
    ctx.data()
        .config
        .common
        .reported_messages_channel
        .to_guild_channel(&ctx, message.guild_id)
        .await?
        .send_message(
            &ctx.http(),
            serenity::CreateMessage::new()
                .content(message.link().to_string())
                .embed(
                    serenity::CreateEmbed::new()
                        .color(ctx.data().config.common.primary_color)
                        .author(
                            serenity::CreateEmbedAuthor::new(message.author.display_name())
                                .icon_url(&message.author.face())
                                .url(format!("https://discord.com/users/{}", &message.author.id)),
                        )
                        .description(message.content),
                ),
        )
        .await?;

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .color(ctx.data().config.common.primary_color)
                .title("Signalement")
                .description(
                    "Merci ! Le message a bien été signalé et sera examiné par un modérateur.",
                ),
        ),
    )
    .await?;

    Ok(())
}
