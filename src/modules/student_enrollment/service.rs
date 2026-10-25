use poise::serenity_prelude as serenity;

use super::{
    config::{REVIEW_CHANNEL, STUDENT_ROLE},
    modals,
    structs::Enrollment,
};
use crate::{Data, Error, config::PRIMARY_COLOR};

async fn first_step(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
) -> Result<Option<Enrollment>, Error> {
    let Some(modal_data) = poise::execute_modal_on_component_interaction::<modals::FirstStep>(
        ctx,
        interaction.clone(),
        None,
        None,
    )
    .await?
    else {
        return Ok(None);
    };

    Ok(Some(Enrollment {
        first_name: crate::utils::title_case(modal_data.first_name.as_str()),
        last_name: modal_data.last_name.to_uppercase(),
        sex: modal_data.sex.as_str().into(),
        level: modal_data.level[0].as_str().into(),
        regime: modal_data.regime.as_str().into(),
        lv1: None,
        lv2: None,
        specialized_subjects: None,
        option: None,
    }))
}

async fn send_for_review(
    ctx: &serenity::Context,
    guild_id: Option<serenity::GuildId>,
    user: &serenity::User,
    enrollment: &Enrollment,
) -> Result<(), Error> {
    REVIEW_CHANNEL
        .to_guild_channel(&ctx.http, guild_id)
        .await?
        .send_message(
            &ctx.http,
            serenity::CreateMessage::new()
                .content("@everyone")
                .embed(
                    serenity::CreateEmbed::new()
                        .color(PRIMARY_COLOR)
                        .title("Nouvelle demande d'inscription !")
                        .field("Prénom", enrollment.first_name.as_str(), true)
                        .field("Nom de famille", enrollment.last_name.as_str(), true)
                        .field("Sexe", enrollment.sex.to_string(), true)
                        .field("Niveau", enrollment.level.to_string(), true)
                        .field("​", "​", true)
                        .field("Régime", enrollment.regime.to_string(), true)
                        .field("LV1", enrollment.lv1.as_deref().unwrap_or("N/A"), true)
                        .field("​", "​", true)
                        .field("LV2", enrollment.lv2.as_deref().unwrap_or("N/A"), true)
                        .field(
                            "Spécialités",
                            enrollment
                                .specialized_subjects
                                .as_ref()
                                .filter(|specialized_subjects| !specialized_subjects.is_empty())
                                .map(|subjects| subjects.join(", "))
                                .unwrap_or_else(|| "N/A".to_string()),
                            true,
                        )
                        .field(
                            "Option",
                            enrollment.option.as_deref().unwrap_or("N/A"),
                            true,
                        ),
                )
                .button(
                    serenity::CreateButton::new(format!(
                        "student_enrollment:review:accept:{}",
                        &user.id
                    ))
                    .emoji(serenity::EmojiId::new(1563129522847752334))
                    .label("Accepter")
                    .style(serenity::ButtonStyle::Success),
                )
                .button(
                    serenity::CreateButton::new(format!(
                        "student_enrollment:review:refuse:{}",
                        &user.id
                    ))
                    .emoji(serenity::EmojiId::new(1563129497967001670))
                    .label("Refuser")
                    .style(serenity::ButtonStyle::Danger),
                ),
        )
        .await?;

    Ok(())
}

pub async fn handle(
    ctx: &serenity::Context,
    _data: &Data,
    interaction: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    if is_student_member(interaction) {
        send_already_enrolled_response(ctx, interaction).await?;
        return Ok(());
    }

    let Some(mut enrollment) = first_step(ctx, interaction).await? else {
        return Ok(());
    };
    let Some(click) = prompt_continue(ctx, interaction).await? else {
        return Ok(());
    };
    let Some(_) = final_step(ctx, &click, &mut enrollment).await? else {
        return Ok(());
    };

    send_for_review(ctx, interaction.guild_id, &interaction.user, &enrollment).await?;
    send_enrollment_success_followup(ctx, interaction, &enrollment).await?;

    Ok(())
}

fn is_student_member(interaction: &serenity::ComponentInteraction) -> bool {
    interaction
        .member
        .as_deref()
        .is_some_and(|m| m.roles.contains(&STUDENT_ROLE))
}

async fn send_already_enrolled_response(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    interaction
        .create_response(
            &ctx.http,
            serenity::CreateInteractionResponse::Message(
                serenity::CreateInteractionResponseMessage::new()
                    .embed(
                        serenity::CreateEmbed::new()
                            .color(PRIMARY_COLOR)
                            .title("Il y a un petit malentendu...")
                            .description("Tu es déjà compté parmi nos élèves, pas besoin de refaire une demande !"),
                    )
                    .ephemeral(true),
            ),
        )
        .await?;

    Ok(())
}

async fn send_enrollment_success_followup(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    enrollment: &Enrollment,
) -> Result<(), Error> {
    interaction
        .create_followup(
            &ctx.http,
            serenity::CreateInteractionResponseFollowup::new()
                .ephemeral(true)
                .embed(
                    serenity::CreateEmbed::new()
                        .color(PRIMARY_COLOR)
                        .title("Ton inscription en tant qu'élève")
                        .description(format!(
                            "C'est tout bon pour moi, **{}** !",
                            enrollment.first_name
                        )),
                ),
        )
        .await?;

    Ok(())
}

async fn prompt_continue(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
) -> Result<Option<serenity::ComponentInteraction>, Error> {
    let message = interaction.create_followup(
        &ctx.http,
        serenity::CreateInteractionResponseFollowup::new()
            .ephemeral(true)
            .embed(
                serenity::CreateEmbed::new()
                    .color(PRIMARY_COLOR)
                    .title("Ton inscription en tant qu'élève")
                    .description("Il me manque quelques informations sur toi avant de valider ton inscription...")
            )
            .button(
                serenity::CreateButton::new("student_enrollment:continue")
                    .label("Continuer")
                    .style(serenity::ButtonStyle::Primary),
            ),
    ).await?;

    let click = serenity::ComponentInteractionCollector::new(ctx)
        .filter(move |mci| {
            mci.message.id == message.id && mci.data.custom_id == "student_enrollment:continue"
        })
        .timeout(std::time::Duration::from_mins(5))
        .await;

    interaction
        .edit_followup(
            &ctx.http,
            message.id,
            serenity::CreateInteractionResponseFollowup::new().button(
                serenity::CreateButton::new("student_enrollment:continue")
                    .label("Continuer")
                    .style(serenity::ButtonStyle::Primary)
                    .disabled(true),
            ),
        )
        .await?;

    Ok(click)
}

/// Dernière étape : l'élève complète son inscription.
async fn final_step(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    enrollment: &mut Enrollment,
) -> Result<Option<serenity::ModalInteraction>, Error> {
    interaction
        .create_response(
            &ctx.http,
            serenity::CreateInteractionResponse::Modal(modals::final_step(enrollment)),
        )
        .await?;

    let submit = serenity::ModalInteractionCollector::new(ctx)
        .filter(move |mci| mci.data.custom_id == "student_enrollment:final_step")
        .timeout(std::time::Duration::from_mins(10))
        .await;

    if let Some(submit) = &submit {
        submit
            .create_response(&ctx.http, serenity::CreateInteractionResponse::Acknowledge)
            .await?;

        for component in submit.data.components.iter() {
            apply_modal_component_to_enrollment(component, enrollment);
        }
    }

    Ok(submit)
}

fn apply_modal_component_to_enrollment(
    component: &serenity::ModalComponent,
    enrollment: &mut Enrollment,
) {
    let serenity::ModalComponent::Label(label) = component else {
        return;
    };

    let serenity::LabelComponent::SelectMenu(select) = &label.component else {
        return;
    };

    let selected_value = select.values.first().map(|value| value.to_string());
    match select.custom_id.as_str() {
        "lv1" => enrollment.lv1 = selected_value,
        "lv2" => enrollment.lv2 = selected_value,
        "first_specialized_subject" | "second_specialized_subject" => {
            if let Some(value) = selected_value {
                enrollment
                    .specialized_subjects
                    .get_or_insert_with(Vec::new)
                    .push(value);
            }
        }
        "option" => enrollment.option = selected_value,
        _ => {}
    }
}
