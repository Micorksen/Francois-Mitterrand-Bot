use poise::serenity_prelude as serenity;

// La liste des disciplines de recrutement
// pour les professeurs est définie dans le modal.

#[derive(serde::Deserialize)]
pub struct Config {
    pub staff_roles: Vec<serenity::RoleId>,
    pub review_channel: serenity::ChannelId,
    pub response_channel: serenity::ChannelId,
}
