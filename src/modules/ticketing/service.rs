use poise::serenity_prelude as serenity;

use crate::{Data, Error};

pub async fn handle(
    ctx: &serenity::Context,
    _data: &Data,
    interaction: &serenity::ComponentInteraction,
) -> Result<(), Error> {
    // TODO

    Ok(())
}
