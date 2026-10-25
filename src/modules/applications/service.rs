use poise::serenity_prelude as serenity;

use super::{
    config::{REVIEW_CHANNEL, STAFF_ROLES},
    modals,
};
use crate::{config::PRIMARY_COLOR, Data, Error};

pub async fn handle(
    ctx: &serenity::Context,
    _data: &Data,
    component: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    if component
        .member
        .as_ref()
        .is_some_and(|m| m.roles.iter().any(|r| STAFF_ROLES.contains(r)))
    {
        component.create_response(
            &ctx.http,
            serenity::CreateInteractionResponse::Message(
                serenity::CreateInteractionResponseMessage::new()
                    .embed(
                        serenity::CreateEmbed::new()
                            .color(PRIMARY_COLOR)
                            .title("Il y a un petit malentendu...")
                            .description("Tu es déjà compté parmi nos membres de l'équipe, pas besoin de refaire une demande !")
                    )
                    .ephemeral(true),
            ),
        ).await?;
        return Ok(());
    }

    let serenity::ComponentInteractionDataKind::StringSelect { values } = &component.data.kind
    else {
        return Ok(());
    };
    let role = &values[0];

    // Affiche un modal spécifique pour les professeurs (choix de la discipline).
    let applicant = if role == "Professeur" {
        let Some(d) = poise::execute_modal_on_component_interaction::<modals::TeacherApplication>(
            ctx,
            component.clone(),
            None,
            None,
        )
        .await?
        else {
            return Ok(());
        };
        super::structs::Applicant {
            civility: d.civility[0].clone(),
            first_name: crate::utils::title_case(d.first_name.as_str()),
            last_name: d.last_name.to_uppercase(),
            discipline: Some(d.discipline[0].clone()),
            motivation_letter: d.motivation_letter[0].clone(),
        }
    } else {
        let Some(d) = poise::execute_modal_on_component_interaction::<modals::StaffApplication>(
            ctx,
            component.clone(),
            None,
            None,
        )
        .await?
        else {
            return Ok(());
        };
        super::structs::Applicant {
            civility: d.civility[0].clone(),
            first_name: crate::utils::title_case(d.first_name.as_str()),
            last_name: d.last_name.to_uppercase(),
            discipline: None,
            motivation_letter: d.motivation_letter[0].clone(),
        }
    };

    let mut embed = serenity::CreateEmbed::new()
        .color(PRIMARY_COLOR)
        .field("Civilité", &applicant.civility, true)
        .field("Prénom", &applicant.first_name, true)
        .field("Nom de famille", &applicant.last_name, true);

    // On ajoute la discipline dans les champs de l'embed.
    // Pour les autres rôle, on ajoute simplement le rôle.
    if let Some(discipline) = &applicant.discipline {
        embed = embed.field("Discipline", discipline, true);
    } else {
        embed = embed.field("Rôle", role, true);
    }

    REVIEW_CHANNEL
        .to_guild_channel(&ctx, component.guild_id)
        .await?
        .send_message(
            &ctx.http,
            serenity::CreateMessage::new()
                .content("@everyone")
                .embed(embed)
                .button(
                    serenity::CreateButton::new(format!(
                        "applications:review:accept:{}",
                        &component.user.id
                    ))
                    .emoji(serenity::EmojiId::new(1563129522847752334))
                    .label("Accepter")
                    .style(serenity::ButtonStyle::Success),
                )
                .button(
                    serenity::CreateButton::new(format!(
                        "applications:review:refuse:{}",
                        &component.user.id
                    ))
                    .emoji(serenity::EmojiId::new(1563129497967001670))
                    .label("Refuser")
                    .style(serenity::ButtonStyle::Danger),
                )
                .add_file(serenity::CreateAttachment::bytes(
                    applicant.motivation_letter.download().await?,
                    format!(
                        "Lettre de motivation - {} {} {}.pdf",
                        &applicant.civility, &applicant.first_name, &applicant.last_name
                    ),
                )),
        )
        .await?;

    component
        .create_followup(
            &ctx.http,
            serenity::CreateInteractionResponseFollowup::new()
                .ephemeral(true)
                .embed(
                    serenity::CreateEmbed::new()
                        .color(PRIMARY_COLOR)
                        .title("Ta candidature en tant que professeur")
                        .description(format!(
                            "C'est tout bon pour moi, **{} {} {}** !",
                            &applicant.civility, &applicant.first_name, &applicant.last_name
                        )),
                ),
        )
        .await?;

    Ok(())
}
