use poise::serenity_prelude as serenity;
use serenity::Mentionable;

use super::modals;
use crate::{
    modules::student_enrollment::{
        config::{
            ENT_CLASSES, INTERN_ROLE, RESPONSE_CHANNEL, SEX_F_ROLE, SEX_M_ROLE, STUDENT_ROLE,
        },
        structs,
        structs::{Regime, Sex},
    }, Data,
    Error,
};

pub async fn handle(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    let mut embed = serenity::builder::CreateEmbed::from(interaction.message.embeds[0].clone());

    match interaction.data.custom_id.as_str() {
        id if id.starts_with("student_enrollment:review:accept:") => {
            accept(ctx, data, interaction).await?;
            embed = embed.color(serenity::colours::branding::GREEN).footer(
                serenity::CreateEmbedFooter::new(format!(
                    "Acceptée par {} ({})",
                    interaction.user.name, interaction.user.id
                )),
            );
        }
        id if id.starts_with("student_enrollment:review:refuse:") => {
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
    data: &Data,
    interaction: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    let Some(modal_data) = poise::execute_modal_on_component_interaction::<modals::Accept>(
        ctx,
        interaction.clone(),
        None,
        None,
    )
    .await?
    else {
        return Ok(());
    };
    let mut member = interaction
        .guild_id
        .unwrap()
        .member(
            &ctx,
            interaction
                .data
                .custom_id
                .split(":")
                .last()
                .unwrap()
                .parse()?,
        )
        .await?;

    // On récupère les informations de l'élève à partir de l'embed du message.
    let class = modal_data.class[0].to_string();
    let first_name = interaction.message.embeds[0].fields[0].value.as_str();
    let last_name = interaction.message.embeds[0].fields[1].value.as_str();

    // On créé le compte ENT de l'élève.
    let http_client = reqwest::Client::new();
    let ent_token = http_client
        .post(format!("{}/auth/oauth2/token", data.config.ent_url))
        .form(&[
            ("client_id", data.config.ent_client_id.as_str()),
            ("client_secret", data.config.ent_client_secret.as_str()),
            ("grant_type", "client_credentials"),
            (
                "scope",
                "org.entcore.directory.controllers.DirectoryController|createUser",
            ),
        ])
        .send()
        .await?
        .json::<structs::Oauth2Token>()
        .await?;

    let ent_account = http_client
        .post(format!("{}/directory/api/user", data.config.ent_url))
        .bearer_auth(ent_token.access_token)
        .form(&[
            ("firstName", first_name),
            ("lastName", last_name),
            ("type", "Student"),
            (
                "classId",
                ENT_CLASSES.get(class.as_str()).expect("Classe inconnue"),
            ),
            ("structureId", &data.config.ent_structure_id),
            ("birthDate", "2001-01-01"),
        ])
        .send()
        .await?
        .json::<structs::ApiUser>()
        .await?;

    // On attribue automatiquement les rôles en fonction des informations renseignées par l'élève.
    let mut roles = vec![STUDENT_ROLE];
    if Sex::from(interaction.message.embeds[0].fields[2].value.as_str()) == Sex::Male {
        roles.push(SEX_M_ROLE);
    } else {
        roles.push(SEX_F_ROLE);
    }

    if Regime::from(interaction.message.embeds[0].fields[5].value.as_str()) == Regime::Intern {
        roles.push(INTERN_ROLE);
    }

    member
        .add_roles(&ctx.http, &roles, Some("Demande d'inscription acceptée"))
        .await?;

    // On renomme l'élève en suivant le format : Classe - Prénom NOM.
    member
        .edit(
            &ctx.http,
            serenity::EditMember::new()
                .nickname(format!("{}・{} {}", class, first_name, last_name,)),
        )
        .await?;

    RESPONSE_CHANNEL
        .to_guild_channel(&ctx, interaction.guild_id)
        .await?
        .send_message(
            &ctx.http,
            serenity::CreateMessage::new()
                .content(member.mention().to_string())
                .embed(
                    serenity::CreateEmbed::new()
                        .color(serenity::colours::branding::GREEN)
                        .title("Demande d'inscription acceptée !")
                        .description(
                            "Félicitations, ta demande d'inscription a été **acceptée** !",
                        ),
                )
                .button(
                    serenity::CreateButton::new(format!(
                        "student_enrollment:retrieve_ent_account:{}",
                        ent_account.id
                    ))
                    .emoji(serenity::EmojiId::new(1563919436140781568))
                    .label("Récupérer mes identifiants")
                    .style(serenity::ButtonStyle::Primary),
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
                        .title("Demande d'inscription refusée !")
                        .description(data.comment),
                ),
        )
        .await?;

    Ok(())
}
