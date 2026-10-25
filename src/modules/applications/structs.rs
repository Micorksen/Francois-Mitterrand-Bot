use poise::serenity_prelude as serenity;

pub struct Applicant {
    pub civility: String,
    pub first_name: String,
    pub last_name: String,
    pub discipline: Option<String>, // Utilisé uniquement dans le cas des candidatures de professeurs.
    pub motivation_letter: serenity::Attachment,
}
