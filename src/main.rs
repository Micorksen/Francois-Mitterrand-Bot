mod commands;
mod config;
mod events;
mod modules;
mod utils;

use std::str::FromStr;

use poise::serenity_prelude as serenity;

pub struct Data {
    pub config: config::Config,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::ApplicationContext<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Command { error, ctx, .. } => {
            eprintln!("❌ Une erreur est survenue dans la commande : {}", error);
            let mut embed = serenity::builder::CreateEmbed::new()
                .color(serenity::colours::branding::RED)
                .title("Une erreur est survenue lors de l'exécution de la commande...")
                .description("C'est un peu frustrant, mais pas de panique ! Nos développeurs sont déjà sur le coup.");

            // On affiche directement le message d'erreur complet si l'utilisateur est l'un des owners du bot.
            if ctx.framework().options().owners.contains(&ctx.author().id) {
                let details: String = format!("{error:?}")
                    .replace("```", "`\u{200b}``")
                    .chars()
                    .take(4000)
                    .collect();
                embed = embed.description(format!("```rs\n{details}\n```"));
            }

            ctx.send(
                poise::CreateReply::default()
                    .embed(embed)
                    .reply(true)
                    .ephemeral(true),
            )
            .await
            .ok();
        }

        poise::FrameworkError::CommandPanic { payload, ctx, .. } => {
            eprintln!(
                "❌ Une erreur est survenue dans la commande : {:?}",
                payload
            );
            let mut embed = serenity::CreateEmbed::new()
                .color(serenity::colours::branding::RED)
                .title("Une erreur est survenue lors de l'exécution de la commande...")
                .description("C'est un peu frustrant, mais pas de panique ! Nos développeurs sont déjà sur le coup.");

            // On affiche directement le message d'erreur complet si l'utilisateur est l'un des owners du bot.
            if ctx.framework().options().owners.contains(&ctx.author().id) {
                let details: String = format!("{payload:?}")
                    .replace("```", "`\u{200b}``")
                    .chars()
                    .take(4000)
                    .collect();
                embed = embed.description(format!("```rs\n{details}\n```"));
            }

            ctx.send(
                poise::CreateReply::default()
                    .embed(embed)
                    .reply(true)
                    .ephemeral(true),
            )
            .await
            .ok();
        }

        poise::FrameworkError::CooldownHit {
            remaining_cooldown,
            ctx,
            ..
        } => {
            let embed = serenity::CreateEmbed::new()
                .color(serenity::colours::branding::YELLOW)
                .title("Tu pousses le bouchon un peu trop loin François !")
                .description(format!("Tu dois attendre encore {} secondes avant de pouvoir réutiliser cette commande.", remaining_cooldown.as_secs()));

            ctx.send(
                poise::CreateReply::default()
                    .embed(embed)
                    .reply(true)
                    .ephemeral(true),
            )
            .await
            .ok();
        }

        error => {
            let _ = poise::builtins::on_error(error).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let config = config::get();
    let framework = poise::Framework::new(poise::FrameworkOptions {
        commands: commands::list(),
        on_error: |error| Box::pin(on_error(error)),
        ..Default::default()
    });

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::GUILD_PRESENCES
        | serenity::GatewayIntents::MESSAGE_CONTENT;
    let mut client =
        serenity::ClientBuilder::new(serenity::Token::from_str(&config.token).unwrap(), intents)
            .framework(Box::new(framework))
            .event_handler(std::sync::Arc::new(events::Handler))
            .data(std::sync::Arc::new(Data { config }))
            .await
            .expect("❌ Une erreur est survenue lors de la création du client...");

    // On démarre le bot.
    if let Err(e) = client.start().await {
        eprintln!(
            "❌ Une erreur est survenue lors du démarrage du bot : {}",
            e
        );
    }
}
