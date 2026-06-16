use teloxide::prelude::*;
pub mod net;
pub mod bot;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {

        Ok(())
    })
    .await;
}
