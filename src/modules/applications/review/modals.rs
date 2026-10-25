use poise::serenity_prelude as serenity;

#[derive(poise::Modal)]
#[name = "Accepter la candidature"]
pub struct Accept {
    #[text_display = "Le candidat sera convoqué à une date et une heure choisie par tes soins pour passer son épreuve orale."]
    #[name = "Date"]
    #[placeholder = "JJ/MM/AAAA"]
    #[min_length = 10]
    #[max_length = 10]
    pub date: serenity::small_fixed_array::FixedString<u16>,

    #[name = "Heure"]
    #[placeholder = "HH:MM"]
    #[min_length = 5]
    #[max_length = 5]
    pub time: serenity::small_fixed_array::FixedString<u16>,
}

#[derive(poise::Modal)]
#[name = "Refuser la candidature"]
pub struct Refuse {
    #[name = "Commentaire"]
    #[paragraph]
    #[min_length = 20]
    #[max_length = 4000]
    pub comment: serenity::small_fixed_array::FixedString<u16>,
}
