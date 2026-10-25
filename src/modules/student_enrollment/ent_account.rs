use poise::serenity_prelude as serenity;

use super::structs;
use crate::{Data, Error, config::PRIMARY_COLOR};

pub async fn retrieve(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    if !is_request_owner(interaction) {
        send_unauthorized_response(ctx, interaction).await?;
        return Ok(());
    }

    let http_client = reqwest::Client::new();
    let ent_token = fetch_ent_token(&http_client, data).await?;
    let Some(ent_user_id) = extract_ent_user_id(interaction) else {
        return Ok(());
    };
    let ent_account = fetch_ent_account(&http_client, data, ent_token, ent_user_id).await?;

    send_credentials_response(ctx, interaction, &ent_account).await?;
    remove_retrieve_button(ctx, interaction).await?;

    Ok(())
}

fn is_request_owner(interaction: &serenity::ComponentInteraction) -> bool {
    interaction
        .message
        .mentions
        .first()
        .is_some_and(|mention| mention.id == interaction.user.id)
}

async fn send_unauthorized_response(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
) -> Result<(), Error> {
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

    Ok(())
}

async fn fetch_ent_token(
    http_client: &reqwest::Client,
    data: &Data,
) -> Result<structs::Oauth2Token, Error> {
    Ok(http_client
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
        .await?)
}

fn extract_ent_user_id(interaction: &serenity::ComponentInteraction) -> Option<&str> {
    interaction.data.custom_id.split(':').next_back()
}

async fn fetch_ent_account(
    http_client: &reqwest::Client,
    data: &Data,
    ent_token: structs::Oauth2Token,
    ent_user_id: &str,
) -> Result<structs::ApiUser, Error> {
    Ok(http_client
        .get(format!(
            "{}/directory/user/{}",
            data.config.ent_url, ent_user_id
        ))
        .bearer_auth(ent_token.access_token)
        .send()
        .await?
        .json::<structs::ApiUser>()
        .await?)
}

async fn send_credentials_response(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
    ent_account: &structs::ApiUser,
) -> Result<(), Error> {
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
                                format!(
                                    "||`{}`||",
                                    ent_account.activation_code.as_deref().unwrap_or("N/A")
                                ),
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

    Ok(())
}

async fn remove_retrieve_button(
    ctx: &serenity::Context,
    interaction: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    let mut message = interaction.message.clone();
    message
        .edit(&ctx.http, serenity::EditMessage::new().components(&[]))
        .await?;

    Ok(())
}
