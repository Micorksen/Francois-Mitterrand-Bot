use poise::serenity_prelude as serenity;
use serenity::Mentionable;

use crate::{
    Data, Error,
    config::{PRIMARY_COLOR, WELCOME_CHANNEL, WELCOME_ROLE},
};

pub async fn handle(
    ctx: &serenity::Context,
    _data: &Data,
    new_member: &serenity::Member,
) -> Result<(), Error> {
    let (guild_member_count, guild_icon_url) = {
        let guild = ctx.cache.guild(new_member.guild_id).unwrap();
        (
            guild.member_count,
            guild.icon_url().unwrap_or("".to_string()),
        )
    };

    new_member
        .add_role(
            &ctx.http,
            WELCOME_ROLE,
            Some("Rôle attribué automatiquement aux nouveaux membres"),
        )
        .await?;
    WELCOME_CHANNEL.to_guild_channel(&ctx.http, Some(new_member.guild_id))
        .await?
        .send_message(
            &ctx.http,
            serenity::CreateMessage::default()
                .content(new_member.mention().to_string())
                .embed(
                    serenity::CreateEmbed::new()
                        .color(PRIMARY_COLOR)
                        .title("🎉 Bienvenue parmi nous !")
                        .description(vec![
                            "Salut, et bienvenue au **Lycée François Mitterrand** !",
                            "On est contents de te voir ici. Prends le temps de faire connaissance avec la communauté.",
                            "N'hésite pas à nous poser toutes tes questions, on est là pour toi. Bonne découverte !",
                        ].join("\n"))
                        .thumbnail(new_member.face(), None)
                        .footer(
                            serenity::CreateEmbedFooter::new(format!("Nous sommes désormais {} membres !", guild_member_count))
                                .icon_url(guild_icon_url)
                        )
                )
        ).await?;

    Ok(())
}
