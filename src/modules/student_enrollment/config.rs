use std::collections::BTreeMap;

use poise::serenity_prelude as serenity;

#[derive(serde::Deserialize)]
pub struct Config {
    pub student_role: serenity::RoleId,
    pub intern_role: serenity::RoleId,
    pub sex_m_role: serenity::RoleId,
    pub sex_f_role: serenity::RoleId,
    pub review_channel: serenity::ChannelId,
    pub response_channel: serenity::ChannelId,
    pub lv1: BTreeMap<String, String>,
    pub lv2: BTreeMap<String, String>,
    pub specialized_subjects: Vec<BTreeMap<String, String>>,
    pub options: BTreeMap<String, String>,
    pub ent: EntConfig,
}

#[derive(serde::Deserialize)]
pub struct EntConfig {
    pub url: String,
    pub oauth2_client_id: String,
    pub oauth2_client_secret: String,
    pub structure_id: String,
    pub classes: BTreeMap<String, String>,
}
