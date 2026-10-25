use poise::serenity_prelude as serenity;

use crate::{Data, Error};

fn has_supporting_activity(presence: &serenity::Presence) -> bool {
    presence.activities.iter().any(|a| {
        a.state
            .as_deref()
            .is_some_and(|s| s.to_ascii_lowercase().contains("/lycee-fm"))
    })
}

/// Système de soutien (avec le lien du serveur dans le statut Discord).
pub async fn handle(
    ctx: &serenity::Context,
    data: &Data,
    old_data: &Option<serenity::Presence>,
    new_data: &serenity::Presence,
) -> Result<(), Error> {
    let Some(guild_id) = new_data.guild_id else {
        return Ok(());
    };

    // On ignore les membres qui sont déconnectés.
    if new_data.status == serenity::OnlineStatus::Offline {
        return Ok(());
    }

    let is_supporting = has_supporting_activity(new_data);

    // On ne réattribue pas le rôle si le membre a conservé le lien dans son statut.
    if let Some(old_data) = old_data {
        if has_supporting_activity(old_data) == is_supporting {
            return Ok(());
        }
    }

    let member = guild_id.member(&ctx, new_data.user.id).await?;
    if is_supporting && !member.roles.contains(&data.config.common.supporter_role) {
        member
            .add_role(
                &ctx.http,
                data.config.common.supporter_role,
                Some("Membre soutenant le serveur"),
            )
            .await?;
    } else if !is_supporting && member.roles.contains(&data.config.common.supporter_role) {
        member
            .remove_role(
                &ctx.http,
                data.config.common.supporter_role,
                Some("Membre ne soutenant plus le serveur"),
            )
            .await?;
    }

    Ok(())
}
