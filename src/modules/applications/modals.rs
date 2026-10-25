use poise::serenity_prelude as serenity;

#[derive(poise::Modal)]
#[name = "Postuler dans l'équipe"]
pub struct StaffApplication {
    #[name = "Civilité"]
    #[string_select("Monsieur", "Madame", "Mademoiselle")]
    pub civility: serenity::small_fixed_array::FixedArray<String>,

    #[name = "Prénom"]
    #[placeholder = "François"]
    pub first_name: serenity::small_fixed_array::FixedString<u16>,

    #[name = "Nom de famille"]
    #[placeholder = "MITTERRAND"]
    pub last_name: serenity::small_fixed_array::FixedString<u16>,

    #[name = "Lettre de motivation"]
    #[file_upload]
    #[file_types(".doc", ".docx", ".odt", ".pdf")]
    pub motivation_letter: serenity::small_fixed_array::FixedArray<serenity::Attachment>,
}

#[derive(poise::Modal)]
#[name = "Postuler dans l'équipe"]
pub struct TeacherApplication {
    #[name = "Civilité"]
    #[string_select("Monsieur", "Madame", "Mademoiselle")]
    pub civility: serenity::small_fixed_array::FixedArray<String>,

    #[name = "Prénom"]
    #[placeholder = "François"]
    pub first_name: serenity::small_fixed_array::FixedString<u16>,

    #[name = "Nom de famille"]
    #[placeholder = "MITTERRAND"]
    pub last_name: serenity::small_fixed_array::FixedString<u16>,

    #[name = "Discipline"]
    #[string_select(
        "L0080 - Documentation",
        "L0100 - Philosophie",
        "L0202 - Lettres modernes",
        "L0421 - Langues vivantes étrangères : Allemand",
        "L0422 - Langues vivantes étrangères : Anglais",
        "L0426 - Langues vivantes étrangères : Italien",
        "L0429 - Langues vivantes étrangères : Espagnol",
        "L1000 - Histoire-Géographie",
        "L1100 - Sciences Économiques et Sociales",
        "L1300 - Mathématiques",
        "L1400 - Technologie",
        "L1500 - Sciences Physiques et Chimiques",
        "L1600 - Sciences de la Vie et de la Terre",
        "L6200 - Numérique et Sciences Informatiques"
    )]
    pub discipline: serenity::small_fixed_array::FixedArray<String>,

    #[name = "Lettre de motivation"]
    #[file_upload]
    #[file_types(".pdf")]
    pub motivation_letter: serenity::small_fixed_array::FixedArray<serenity::Attachment>,
}
