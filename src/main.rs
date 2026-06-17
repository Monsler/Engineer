use std::sync::Arc;

use teloxide::prelude::*;

use crate::net::G4Free;
pub mod net;
pub mod bot;

async fn dispatch_message(bot: &Bot, message: &str, engineer: &bot::Engineer<'_, '_>) {
    if message.starts_with("/report") {

    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = Bot::from_env();
    let core = Arc::new(bot::Engineer::from(Box::new(G4Free::new("api_key"))));

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let clone = Arc::clone(&core);

        async move {
            if let Some(input) = msg.text() {
                dispatch_message(&bot, input, &clone).await;
            }

            Ok(())
        }
    })
    .await;
}
