use poise::serenity_prelude as serenity;

use super::{
    config::{REVIEW_CHANNEL, STUDENT_ROLE},
    modals, structs,
    structs::Enrollment,
};
use crate::{config::PRIMARY_COLOR, Data, Error};

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
    if interaction
        .member
        .as_deref()
        .is_some_and(|m| m.roles.contains(&STUDENT_ROLE))
    {
        interaction.create_response(&ctx.http, serenity::CreateInteractionResponse::Message(
            serenity::CreateInteractionResponseMessage::new()
                .embed(
                    serenity::CreateEmbed::new()
                        .color(PRIMARY_COLOR)
                        .title("Il y a un petit malentendu...")
                        .description("Tu es déjà compté parmi nos élèves, pas besoin de refaire une demande !")
                )
                .ephemeral(true),
        )).await?;
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
            let serenity::ModalComponent::Label(label) = component else {
                continue;
            };

            match &label.component {
                serenity::LabelComponent::SelectMenu(select) => {
                    let values: Vec<String> = select
                        .values
                        .iter()
                        .map(|value| value.to_string())
                        .collect();
                    match select.custom_id.as_str() {
                        "lv1" => enrollment.lv1 = values.into_iter().next(),
                        "lv2" => enrollment.lv2 = values.into_iter().next(),
                        "first_specialized_subject" | "second_specialized_subject" => {
                            if let Some(value) = values.into_iter().next() {
                                enrollment
                                    .specialized_subjects
                                    .get_or_insert_with(Vec::new)
                                    .push(value);
                            }
                        }
                        "option" => enrollment.option = values.into_iter().next(),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    Ok(submit)
}

pub async fn retrieve_ent_account(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    if interaction.message.mentions[0].id != interaction.user.id {
        interaction
            .create_response(
                &ctx.http,
                serenity::CreateInteractionResponse::Message(
                    serenity::CreateInteractionResponseMessage::new()
                        .ephemeral(true)
                        .embed(
                            serenity::CreateEmbed::new()
                                .color(PRIMARY_COLOR)
                                .title("Récupérer mes identifiants")
                                .description("Ce message ne t'était pas destiné..."),
                        ),
                ),
            )
            .await?;

        return Ok(());
    }

    let http_client = reqwest::Client::new();

    let ent_token = http_client
        .post(format!("{}/auth/oauth2/token", data.config.ent_url))
        .form(&[
            ("client_id", data.config.ent_client_id.as_str()),
            ("client_secret", data.config.ent_client_secret.as_str()),
            ("grant_type", "client_credentials"),
            (
                "scope",
                "org.entcore.directory.controllers.UserController|get",
            ),
        ])
        .send()
        .await?
        .json::<structs::Oauth2Token>()
        .await?;

    let ent_account = http_client
        .get(format!(
            "{}/directory/user/{}",
            data.config.ent_url,
            interaction.data.custom_id.split(":").last().unwrap()
        ))
        .bearer_auth(ent_token.access_token)
        .send()
        .await?
        .json::<structs::ApiUser>()
        .await?;

    interaction
        .create_response(
            &ctx.http,
            serenity::CreateInteractionResponse::Message(
                serenity::CreateInteractionResponseMessage::new()
                    .ephemeral(true)
                    .embed(
                        serenity::CreateEmbed::new()
                            .color(PRIMARY_COLOR)
                            .title("Récupérer mes identifiants")
                            .field("Identifiant", format!("`{}`", ent_account.login), true)
                            .field(
                                "Code d'activation",
                                format!("||`{}`||", ent_account.activation_code.unwrap()),
                                true,
                            ),
                    )
                    .button(
                        serenity::CreateButton::new_link("https://ent.lycee-fm.fr/")
                            .emoji(serenity::EmojiId::new(1563919436140781568))
                            .label("Accéder à l'ENT"),
                    ),
            ),
        )
        .await?;

    // On retire le bouton pour récupérer les identifiants
    // après le premier clic.
    let mut message = interaction.message.clone();
    message
        .edit(&ctx.http, serenity::EditMessage::new().components(&[]))
        .await?;

    Ok(())
}
