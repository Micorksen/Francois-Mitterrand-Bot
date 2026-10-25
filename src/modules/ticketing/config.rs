use poise::serenity_prelude as serenity;

pub const CATEGORY: serenity::ChannelId = serenity::ChannelId::new(1563272520722030593);
pub const SUPPORT_ROLES: &[serenity::RoleId] = &[
    serenity::RoleId::new(1550774020092207144), // Proviseur
    serenity::RoleId::new(1550774021442904084), // Proviseur Adjoint
    serenity::RoleId::new(1550774023506501682), // Assistant de Direction
    serenity::RoleId::new(1550774024471187496), // Secrétaire
    serenity::RoleId::new(1550774025469558795), // Conseiller Principal d'Éducation
    serenity::RoleId::new(1550774036479610930), // Modérateur
];
