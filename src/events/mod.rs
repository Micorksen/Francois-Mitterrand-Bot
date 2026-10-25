mod interaction_create;
mod member_join;
mod message;
mod presence_update;
mod ready;

use poise::serenity_prelude as serenity;

use crate::Data;

pub struct Handler;

/// S'occupe de gérer les évènements et de les dispatcher correctement.
#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn dispatch(&self, ctx: &serenity::Context, event: &serenity::FullEvent) {
        let data: std::sync::Arc<Data> = ctx.data();
        match event {
            serenity::FullEvent::InteractionCreate { interaction, .. } => {
                if let serenity::Interaction::Component(component) = interaction {
                    let _ = interaction_create::handle_component(ctx, &data, component).await;
                }
            }
            serenity::FullEvent::Ready { data_about_bot, .. } => {
                let _ = ready::handle(ctx, &data, data_about_bot).await;
            }
            serenity::FullEvent::GuildMemberAddition { new_member, .. } => {
                let _ = member_join::handle(ctx, &data, new_member).await;
            }
            serenity::FullEvent::PresenceUpdate {
                old_data, new_data, ..
            } => {
                let _ = presence_update::handle(ctx, &data, &old_data, &new_data).await;
            }
            serenity::FullEvent::Message { new_message, .. } => {
                let _ = message::handle(ctx, &data, &new_message).await;
            }
            _ => {}
        }
    }
}
