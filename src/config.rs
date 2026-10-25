use poise::serenity_prelude as serenity;

/// Couleur primaire utilisée pour les embeds.
pub const PRIMARY_COLOR: u32 = 0x3A2D81;

pub const HONEYPOT_CHANNEL: serenity::ChannelId = serenity::ChannelId::new(1563132033864040578);
pub const REPORTED_MESSAGES_CHANNEL: serenity::ChannelId =
    serenity::ChannelId::new(1563295003722260641);
pub const WELCOME_CHANNEL: serenity::ChannelId = serenity::ChannelId::new(1550774192490680372);

pub const SUPPORTER_ROLE: serenity::RoleId = serenity::RoleId::new(1550774050689912903);
pub const WELCOME_ROLE: serenity::RoleId = serenity::RoleId::new(1550774068855177247);

#[derive(Clone, serde::Deserialize)]
pub struct Config {
    #[serde(rename = "bot_token")]
    pub token: String,
    #[serde(default)]
    pub environment: String,

    pub ent_url: String,
    pub ent_client_id: String,
    pub ent_client_secret: String,
    pub ent_structure_id: String,
}

pub fn get() -> Config {
    dotenvy::dotenv().ok();
    envy::from_env::<Config>().expect("❌ Configuration invalide.")
}
