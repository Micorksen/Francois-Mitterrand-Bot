use poise::serenity_prelude as serenity;

#[derive(poise::Modal)]
#[name = "Accepter la demande d'inscription"]
pub struct Accept {
    #[text_display = "Les accès OXONET de l'élève sont créés automatiquement. Cependant, tu dois t'assurer que l'élève est référencé sur la base PRONOTE."]
    #[name = "Classe"]
    #[string_select("2A", "2B", "2C", "1A", "1B", "1C", "TA", "TB", "TC")]
    pub class: serenity::small_fixed_array::FixedArray<String>,
}

#[derive(poise::Modal)]
#[name = "Refuser la demande d'inscription"]
pub struct Refuse {
    #[name = "Commentaire"]
    #[paragraph]
    #[min_length = 20]
    #[max_length = 4000]
    pub comment: serenity::small_fixed_array::FixedString<u16>,
}
