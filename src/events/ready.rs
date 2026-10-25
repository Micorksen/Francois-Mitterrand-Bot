use poise::serenity_prelude as serenity;

use crate::{commands, Data, Error};

/// Constante utilisée pour éviter d'enregistrer deux fois les commandes.
static COMMANDS_REGISTERED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

pub async fn handle(
    ctx: &serenity::Context,
    data: &Data,
    data_about_bot: &serenity::Ready,
) -> Result<(), Error> {
    // On propose un mode d'environnement de « développement ».
    // Dans ce cas, le bot modifie son statut pour l'expliciter.
    if ["dev", "development"].contains(&data.config.environment.as_str()) {
        let activity =
            serenity::ActivityData::playing("Je suis en train de revoir mon gouvernement...");
        ctx.set_presence(Some(activity), serenity::OnlineStatus::DoNotDisturb);
    } else {
        let activity = serenity::ActivityData::playing("Client PRONOTE 2026");
        ctx.set_presence(Some(activity), serenity::OnlineStatus::Online);
    }

    println!("✅ Connecté en tant que {} !", data_about_bot.user.name);

    // On enregistre les commandes (/).
    if !COMMANDS_REGISTERED.swap(true, std::sync::atomic::Ordering::SeqCst) {
        match poise::builtins::register_globally(&ctx.http, &commands::list()).await {
            Ok(()) => println!("✅ L'enregistrement des commandes (/) s'est fait avec succès !"),
            Err(e) => {
                COMMANDS_REGISTERED.store(false, std::sync::atomic::Ordering::SeqCst);
                eprintln!(
                    "❌ Une erreur est survenue lors de l'enregistrement des commandes (/) : {e}"
                );
            }
        }
    }

    Ok(())
}
