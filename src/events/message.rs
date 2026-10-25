use poise::serenity_prelude as serenity;

use crate::{config::HONEYPOT_CHANNEL, Data, Error};

/// Système du pot de miel (bannissement automatique quand un message est envoyé dans un salon spécifique).
pub async fn handle(
    ctx: &serenity::Context,
    _data: &Data,
    message: &serenity::Message,
) -> Result<(), Error> {
    if message.guild_channel(&ctx.http).await?.id != HONEYPOT_CHANNEL {
        return Ok(());
    }

    message
        .guild_id
        .unwrap()
        .member(&ctx.http, message.author.id)
        .await?
        .ban(
            &ctx.http,
            604800,
            Some("Banni automatiquement pour avoir envoyé un message dans le salon honeypot"),
        )
        .await?;

    Ok(())
}
