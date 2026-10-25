use poise::serenity_prelude as serenity;

use crate::{Data, Error};

pub async fn handle_component(
    ctx: &serenity::Context,
    data: &Data,
    component: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    match component.data.custom_id.as_str() {
        // Système de recrutement des membres du personnel.
        "applications:begin" => {
            crate::modules::applications::service::handle(ctx, data, component).await
        }
        id if id.starts_with("applications:review:") => {
            crate::modules::applications::review::service::handle(ctx, data, component).await
        }

        // Système d'inscription des élèves.
        "student_enrollment:begin" => {
            crate::modules::student_enrollment::service::handle(ctx, data, component).await
        }
        id if id.starts_with("student_enrollment:review:") => {
            crate::modules::student_enrollment::review::service::handle(ctx, data, component).await
        }
        id if id.starts_with("student_enrollment:retrieve_ent_account:") => {
            crate::modules::student_enrollment::ent_account::retrieve(ctx, data, component).await
        }

        _ => Ok(()),
    }
}
