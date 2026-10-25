use poise::serenity_prelude as serenity;

use crate::{Data, Error};

/// Système du honeypot (bannissement automatique quand un message est envoyé dans un salon spécifique).
pub async fn handle(
    ctx: &serenity::Context,
    data: &Data,
    message: &serenity::Message,
) -> Result<(), Error> {
    if message.channel_id != data.config.common.honeypot_channel.widen() {
        return Ok(());
    }

    message
        .guild_id
        .unwrap()
        .ban(
            &ctx.http,
            message.author.id,
            604800,
            Some("Banni automatiquement pour avoir envoyé un message dans le salon honeypot"),
        )
        .await?;

    Ok(())
}
