use poise::serenity_prelude as serenity;

use crate::{Data, Error, config::SUPPORTING_ROLE};

/// Système de soutien (avec le lien du serveur dans le statut Discord).
pub async fn handle(
    ctx: &serenity::Context,
    _data: &Data,
    _old_data: &Option<serenity::Presence>,
    new_data: &serenity::Presence,
) -> Result<(), Error> {
    let Some(guild_id) = new_data.guild_id else {
        return Ok(());
    };
    let member = guild_id.member(&ctx.http, new_data.user.id).await?;

    let is_supporting = new_data.activities.iter().any(|a| {
        a.state
            .as_deref()
            .is_some_and(|s| s.to_lowercase().contains("/lycee-fm"))
    });

    if is_supporting && !member.roles.contains(&SUPPORTING_ROLE) {
        member
            .add_role(
                &ctx.http,
                SUPPORTING_ROLE,
                Some("Rôle attribué aux personnes soutenant le serveur"),
            )
            .await?;
    } else {
        member
            .remove_role(
                &ctx.http,
                SUPPORTING_ROLE,
                Some("Rôle attribué aux personnes soutenant le serveur"),
            )
            .await?;
    }

    Ok(())
}
