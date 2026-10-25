use poise::serenity_prelude as serenity;

#[derive(serde::Deserialize)]
pub struct Config {
    pub category: serenity::ChannelId,
    pub support_roles: Vec<serenity::RoleId>,
}
