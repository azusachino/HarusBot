use teloxide::{prelude::*, utils::command::BotCommands};

mod memo;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "please check all cmds")]
pub enum Command {
    #[command(description = "help not yet supported")]
    Help,
    #[command(description = "deliver your instant memory")]
    Memo(String),
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Memo(memo) => {
            memo::handle_memo(memo)
                .await
                .expect("fail to handle memo command");
            bot.send_message(msg.chat.id, format!("Your memo has been collected."))
                .await?
        }
    };

    Ok(())
}
