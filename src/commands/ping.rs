use poise::serenity_prelude as serenity;

use crate::{Context, Error};

/// Vérifie si le bot répond.
#[poise::command(slash_command, ephemeral)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::default()
                .color(ctx.data().config.common.primary_color)
                .title("Pong !")
                .description("La mort ne m'a pas encore assassiné. Une petite pensée à Marcia..."),
        ),
    )
    .await?;

    Ok(())
}
