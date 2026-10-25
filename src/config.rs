use figment::providers::Format;
use poise::serenity_prelude as serenity;

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(rename = "bot_token")]
    pub token: String,
    #[serde(default)]
    pub environment: String,

    pub common: CommonConfig,
    pub applications: crate::modules::applications::config::Config,
    pub student_enrollment: crate::modules::student_enrollment::config::Config,
    pub ticketing: crate::modules::ticketing::config::Config,
}

#[derive(serde::Deserialize)]
pub struct CommonConfig {
    pub primary_color: serenity::Color,
    pub honeypot_channel: serenity::ChannelId,
    pub reported_messages_channel: serenity::ChannelId,
    pub welcome_channel: serenity::ChannelId,
    pub supporter_role: serenity::RoleId,
    pub welcome_role: serenity::RoleId,
}

pub fn get() -> Config {
    dotenvy::dotenv().ok();

    figment::Figment::new()
        .merge(figment::providers::Toml::file("config/common.toml"))
        .merge(figment::providers::Toml::file("config/applications.toml"))
        .merge(figment::providers::Toml::file(
            "config/student_enrollment.toml",
        ))
        .merge(figment::providers::Toml::file("config/ticketing.toml"))
        .merge(figment::providers::Env::raw().only(&["BOT_TOKEN", "ENVIRONMENT"]))
        .merge(figment::providers::Serialized::default(
            "student_enrollment.ent.oauth2_client_secret",
            std::env::var("ENT_OAUTH2_CLIENT_SECRET")
                .expect("❌ Configuration invalide : La variable d'environnement ENT_OAUTH2_CLIENT_SECRET est manquante."),
        ))
        .extract()
        .unwrap_or_else(|e| panic!("❌ Configuration invalide : {e}"))
}
