use crate::{Data, Error};

mod embed;
mod ping;
mod signaler_un_message;

/// Renvoie la liste des commandes à charger.
pub fn list() -> Vec<poise::Command<Data, Error>> {
    vec![
        embed::embed(),
        ping::ping(),
        signaler_un_message::signaler_un_message(),
    ]
}
