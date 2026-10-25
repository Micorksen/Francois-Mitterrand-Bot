use poise::serenity_prelude as serenity;

// La liste des disciplines de recrutement
// pour les professeurs est définie dans le modal.

pub const STAFF_ROLES: &[serenity::RoleId] = &[
    serenity::RoleId::new(1550774020092207144), // Proviseur
    serenity::RoleId::new(1550774021442904084), // Proviseur Adjoint
    serenity::RoleId::new(1550774023506501682), // Assistant de Direction
    serenity::RoleId::new(1550774024471187496), // Secrétaire
    serenity::RoleId::new(1550774025469558795), // Conseiller Principal d'Éducation
    serenity::RoleId::new(1550774006804783205), // Professeur
    serenity::RoleId::new(1550774039264370800), // Assistant d'éducation
    serenity::RoleId::new(1550774041650921473), // Infirmier
    serenity::RoleId::new(1550774036479610930), // Modérateur
];

pub const REVIEW_CHANNEL: serenity::ChannelId = serenity::ChannelId::new(1562936388385640529);
pub const RESPONSE_CHANNEL: serenity::ChannelId = serenity::ChannelId::new(1563158109231513671);
