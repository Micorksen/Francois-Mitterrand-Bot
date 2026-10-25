use poise::serenity_prelude as serenity;

use crate::{config::PRIMARY_COLOR, Context, Error};

#[derive(poise::ChoiceParameter)]
pub enum EmbedChoice {
    #[name = "Absences"]
    Absences,
    #[name = "CDI numérique"]
    CdiNumerique,
    #[name = "Pot de miel"]
    PotDeMiel,
    #[name = "PRONOTE"]
    Pronote,
    #[name = "Rejoindre l'aventure"]
    JoinTheAdventure,
    #[name = "Règlement"]
    Rules,
    #[name = "Tickets"]
    Tickets,
}

impl EmbedChoice {
    pub fn json(&self) -> &'static str {
        match self {
            Self::Absences => include_str!("../../embeds/absences.json"),
            Self::CdiNumerique => include_str!("../../embeds/cdi_numerique.json"),
            Self::PotDeMiel => include_str!("../../embeds/pot_de_miel.json"),
            Self::Pronote => include_str!("../../embeds/pronote.json"),
            Self::JoinTheAdventure => include_str!("../../embeds/join_the_adventure.json"),
            Self::Rules => include_str!("../../embeds/rules.json"),
            Self::Tickets => include_str!("../../embeds/tickets.json"),
        }
    }
}

/// Affiche un message enrichi pré-conçu.
#[poise::command(slash_command, ephemeral, default_member_permissions = "ADMINISTRATOR")]
pub async fn embed(
    ctx: Context<'_>,
    #[description = "Le message enrichi à afficher"] message_enrichi: EmbedChoice,
    #[description = "Le salon où envoyer le message enrichi"] salon: Option<serenity::GuildChannel>,
) -> Result<(), Error> {
    let mut message: serde_json::Value = serde_json::from_str(message_enrichi.json())?;
    if let Some(embeds) = message.get_mut("embeds").and_then(|e| e.as_array_mut()) {
        for embed in embeds {
            if let Some(obj) = embed.as_object_mut() {
                // Si le message contient un ou plusieurs embeds, et qu'il n'explicite
                // pas une couleur, on utilise la couleur primaire.
                obj.entry("color")
                    .or_insert(serde_json::json!(PRIMARY_COLOR));
            }
        }
    }

    if let Some(salon) = salon {
        ctx.http()
            .send_message(
                serenity::GenericChannelId::from(salon.id),
                Vec::new(),
                &message,
            )
            .await?;
    } else {
        ctx.http()
            .send_message(ctx.channel_id(), Vec::new(), &message)
            .await?;
    }

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .color(PRIMARY_COLOR)
                .title("Message enrichi")
                .description("Le message enrichi a été envoyé avec **succès** !"),
        ),
    )
    .await?;

    Ok(())
}
