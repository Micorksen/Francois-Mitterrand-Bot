use poise::serenity_prelude as serenity;
use serenity::Mentionable;

use crate::{Data, Error};

pub async fn handle(
    ctx: &serenity::Context,
    data: &Data,
    new_member: &serenity::Member,
) -> Result<(), Error> {
    // Ajoute le rôle de bienvenue aux nouveaux membres.
    new_member
        .add_role(
            &ctx.http,
            data.config.common.welcome_role,
            Some("Rôle attribué automatiquement aux nouveaux membres"),
        )
        .await?;

    // Récupère le nombre de membres et l'icône du serveur (en privilégiant le cache).
    let (member_count, icon_url) = if let Some(guild) = ctx.cache.guild(new_member.guild_id) {
        (guild.member_count, guild.icon_url().unwrap_or_default())
    } else {
        let guild = ctx.http.get_guild_with_counts(new_member.guild_id).await?;
        (
            guild.approximate_member_count.unwrap(),
            guild.icon_url().unwrap_or_default(),
        )
    };

    data.config.common.welcome_channel.to_guild_channel(&ctx, Some(new_member.guild_id))
        .await?
        .send_message(
            &ctx.http,
            serenity::CreateMessage::default()
                .content(new_member.mention().to_string())
                .embed(
                    serenity::CreateEmbed::new()
                        .color(data.config.common.primary_color)
                        .title("🎉 Bienvenue parmi nous !")
                        .description(vec![
                            "Salut, et bienvenue au **Lycée François Mitterrand** !",
                            "On est contents de te voir ici. Prends le temps de faire connaissance avec la communauté.",
                            "N'hésite pas à nous poser toutes tes questions, on est là pour toi. Bonne découverte !",
                        ].join("\n"))
                        .thumbnail(new_member.face(), None)
                        .footer(
                            serenity::CreateEmbedFooter::new(format!("Nous sommes désormais {} membres !", member_count))
                                .icon_url(icon_url)
                        )
                )
        ).await?;

    Ok(())
}
