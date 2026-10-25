use poise::serenity_prelude as serenity;

/// Rôle attribué aux élèves.
pub const STUDENT_ROLE: serenity::RoleId = serenity::RoleId::new(1550774060718358570);

/// Rôle attribué aux élèves internes.
pub const INTERN_ROLE: serenity::RoleId = serenity::RoleId::new(1550774065210593322);

/// Rôle attribué aux élèves de sexe masculin.
pub const SEX_M_ROLE: serenity::RoleId = serenity::RoleId::new(1550774076799197254);

/// Rôle attribué aux élèves de sexe féminin.
pub const SEX_F_ROLE: serenity::RoleId = serenity::RoleId::new(1550774079727079535);

/// Salon à utiliser pour envoyer les inscriptions.
pub const REVIEW_CHANNEL: serenity::ChannelId = serenity::ChannelId::new(1555219169782276216);

/// Salon à utiliser pour envoyer les réponses aux candidatures. TODO: à changer
pub const RESPONSE_CHANNEL: serenity::ChannelId = serenity::ChannelId::new(1563247667818135582);

/// Liste des langues vivantes 1.
pub const LV1: &[(&str, &str)] = &[("AGL1", "Anglais LV1")];

/// Liste des langues vivantes 2.
pub const LV2: &[(&str, &str)] = &[
    ("ALL2", "Allemand LV2"),
    ("ESP2", "Espagnol LV2"),
    ("ITA2", "Italien LV2"),
];

/// Liste des matières de spécialité, regroupés en deux groupes.
pub const SPECIALIZED_SUBJECTS: &[&[(&str, &str)]] = &[
    &[
        ("AMC", "Anglais Monde Contemporain"),
        (
            "HGGSP",
            "Histoire-Géographie, Géopolitique et Sciences Politiques",
        ),
        ("HLP", "Humanités, Littérature et Philosophie"),
        ("SES", "Sciences Économiques et Sociales"),
    ],
    &[
        ("MATHS", "Mathématiques"),
        ("NSI", "Numérique et Sciences Informatiques"),
        ("PH-CH", "Physique-Chimie"),
        ("SI", "Sciences de l'Ingénieur"),
        ("SVT", "Sciences de la Vie et de la Terre"),
    ],
];

/// Liste des options proposées.
pub const OPTIONS: &[(&str, &str)] = &[
    ("DGEMC", "Droit et grands enjeux du monde contemporain"),
    ("MATH.COMPL", "Mathématiques complémentaires"),
    ("MATH.EXPERT", "Mathématiques expertes"),
];

/// Listes des classes sur l'ENT.
pub const ENT_CLASSES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "2A" => "22a3b6d7-dda8-4f1e-8006-a9964dcfff2f",
    "2B" => "7e77dc75-daf4-4479-9e44-358d76d57eff",
    "2C" => "60daefb2-3f27-4213-b1c3-6510fa742e0a",
    "1A" => "4b970755-fc99-49fc-bb28-a7dc2ea45f8a",
    "1B" => "65b64f54-aca6-41c8-baf4-bab2bacda4d7",
    "1C" => "519b1566-c423-4890-9911-51147d3cccd2",
    "TA" => "943517f1-d068-4496-b424-a517982187d5",
    "TB" => "ec4f0568-6ca7-49e0-9009-96efcdec25ea",
    "TC" => "ca3f4eac-de2e-4eb6-a770-b6b1ee9e73bf",
};
