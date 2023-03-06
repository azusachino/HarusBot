use harus_bot::{answer, Command, Result};
use teloxide::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("starting harus-tg-bot...");

    dotenv::dotenv()?;

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;

    Ok(())
}
