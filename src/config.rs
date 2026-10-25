use poise::serenity_prelude as serenity;

/// Couleur primaire utilisée pour les embeds.
pub const PRIMARY_COLOR: u32 = 0x3A2D81;

/// Salon utilisé pour accueillir les nouveaux membres.
pub const WELCOME_CHANNEL: serenity::ChannelId = serenity::ChannelId::new(1550774192490680372);

/// Salon utilisé pour répertorier les messages signalés.
pub const REPORTED_MESSAGES_CHANNEL: serenity::ChannelId =
    serenity::ChannelId::new(1563295003722260641);

/// Salon utilisé pour le honeypot.
pub const HONEYPOT_CHANNEL: serenity::ChannelId = serenity::ChannelId::new(1563132033864040578);

/// Rôle attribué aux nouveaux membres.
pub const WELCOME_ROLE: serenity::RoleId = serenity::RoleId::new(1550774068855177247);

/// Rôle attribué aux membres faisant la promotion du serveur.
pub const SUPPORTING_ROLE: serenity::RoleId = serenity::RoleId::new(1550774050689912903);

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
